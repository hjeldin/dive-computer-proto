//! Communication protocol serialization/deserialization layer
//!
//! This module provides functionality for encoding and decoding messages
//! for communication between the dive computer and external devices.

use serde::{Serialize, Deserialize};
use core::convert::TryFrom;

/// Maximum message size in bytes
pub const MAX_MESSAGE_SIZE: usize = 256;

/// Protocol version
pub const PROTOCOL_VERSION: u8 = 1;

/// Message header magic bytes for identifying valid messages
pub const MESSAGE_MAGIC: [u8; 2] = [0xDC, 0x42]; // DC = Dive Computer, 42 = "the answer"

/// Error types for protocol operations
#[derive(Debug, Eq, PartialEq)]
pub enum ProtocolError {
    /// Message is too large to fit in the buffer
    MessageTooLarge,
    /// Invalid message format
    InvalidFormat,
    /// Checksum verification failed
    ChecksumMismatch,
    /// Invalid magic bytes in header
    InvalidMagic,
    /// Unsupported protocol version
    UnsupportedVersion,
    /// Serialization error
    SerializationError,
    /// Deserialization error
    DeserializationError,
}

/// Message types that can be sent or received
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum MessageKind {
    /// Command message
    Command = 0x01,
    /// Response message
    Response = 0x02,
    /// Notification message (sent without a command)
    Notification = 0x03,
    /// Acknowledgment message
    Ack = 0x04,
    /// Error message
    Error = 0x05,
}

/// Message header containing metadata about the message
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageHeader {
    /// Magic bytes to identify valid messages
    pub magic: [u8; 2],
    /// Protocol version
    pub version: u8,
    /// Message type
    pub kind: MessageKind,
    /// Message sequence number for matching requests and responses
    pub sequence: u16,
    /// Length of the payload in bytes
    pub payload_length: u16,
    /// Checksum of the header (excluding the checksum field itself)
    pub header_checksum: u8,
}

impl MessageHeader {
    /// Creates a new message header
    pub fn new(kind: MessageKind, sequence: u16, payload_length: u16) -> Self {
        let mut header = MessageHeader {
            magic: MESSAGE_MAGIC,
            version: PROTOCOL_VERSION,
            kind,
            sequence,
            payload_length,
            header_checksum: 0, // Will be calculated below
        };
        
        header.header_checksum = calculate_checksum(&[
            header.magic[0], header.magic[1],
            header.version,
            header.kind as u8,
            (header.sequence >> 8) as u8, header.sequence as u8,
            (header.payload_length >> 8) as u8, header.payload_length as u8,
        ]);
        
        header
    }
    
    /// Validates the header checksum
    pub fn validate_checksum(&self) -> bool {
        let calculated = calculate_checksum(&[
            self.magic[0], self.magic[1],
            self.version,
            self.kind as u8,
            (self.sequence >> 8) as u8, self.sequence as u8,
            (self.payload_length >> 8) as u8, self.payload_length as u8,
        ]);
        
        calculated == self.header_checksum
    }
}

/// A complete message with header and payload
#[derive(Debug)]
pub struct Message<T> {
    /// Message header
    pub header: MessageHeader,
    /// Message payload
    pub payload: T,
    /// Payload checksum
    pub payload_checksum: u8,
}

impl<T: Serialize> Message<T> {
    /// Creates a new message with the given payload
    pub fn new(kind: MessageKind, sequence: u16, payload: T) -> Result<Self, ProtocolError> {
        // Serialize payload to calculate its length
        let mut payload_buffer = [0u8; MAX_MESSAGE_SIZE];
        let payload_length = match postcard::to_slice(&payload, &mut payload_buffer) {
            Ok(data) => data.len(),
            Err(_) => return Err(ProtocolError::SerializationError),
        };
        
        if payload_length > u16::MAX as usize {
            return Err(ProtocolError::MessageTooLarge);
        }
        
        let header = MessageHeader::new(kind, sequence, payload_length as u16);
        let payload_checksum = calculate_checksum(&payload_buffer[..payload_length]);
        
        Ok(Message {
            header,
            payload,
            payload_checksum,
        })
    }
    
    /// Serializes the message to a byte buffer
    pub fn serialize(&self) -> Result<(usize, [u8; MAX_MESSAGE_SIZE]), ProtocolError> {
        let mut buffer = [0u8; MAX_MESSAGE_SIZE];
        let mut offset = 0;
        
        // Serialize header
        match postcard::to_slice(&self.header, &mut buffer) {
            Ok(data) => offset += data.len(),
            Err(_) => return Err(ProtocolError::SerializationError),
        }
        
        // Serialize payload
        match postcard::to_slice(&self.payload, &mut buffer[offset..]) {
            Ok(data) => offset += data.len(),
            Err(_) => return Err(ProtocolError::SerializationError),
        }
        
        // Add payload checksum
        if offset >= MAX_MESSAGE_SIZE {
            return Err(ProtocolError::MessageTooLarge);
        }
        buffer[offset] = self.payload_checksum;
        offset += 1;
        
        Ok((offset, buffer))
    }
}

impl<'de, T: Deserialize<'de>> TryFrom<&'de [u8]> for Message<T> {
    type Error = ProtocolError;
    
    /// Deserializes a message from a byte buffer
    fn try_from(data: &'de [u8]) -> Result<Self, Self::Error> {
        if data.len() < 9 { // Minimum size for a header
            return Err(ProtocolError::InvalidFormat);
        }
        
        // Deserialize header
        let (header, header_size): (MessageHeader, usize) = match postcard::take_from_bytes(data) {
            Ok((header, rest)) => (header, data.len() - rest.len()),
            Err(_) => return Err(ProtocolError::DeserializationError),
        };
        
        // Validate header
        if header.magic != MESSAGE_MAGIC {
            return Err(ProtocolError::InvalidMagic);
        }
        
        if header.version != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion);
        }
        
        if !header.validate_checksum() {
            return Err(ProtocolError::ChecksumMismatch);
        }
        
        // Check if we have enough data for the payload
        let expected_size = header_size + header.payload_length as usize + 1; // +1 for payload checksum
        if data.len() < expected_size {
            return Err(ProtocolError::InvalidFormat);
        }
        
        // Deserialize payload
        let payload_data = &data[header_size..(header_size + header.payload_length as usize)];
        let payload: T = match postcard::from_bytes(payload_data) {
            Ok(payload) => payload,
            Err(_) => return Err(ProtocolError::DeserializationError),
        };
        
        // Validate payload checksum
        let payload_checksum = data[header_size + header.payload_length as usize];
        let calculated_checksum = calculate_checksum(payload_data);
        
        if calculated_checksum != payload_checksum {
            return Err(ProtocolError::ChecksumMismatch);
        }
        
        Ok(Message {
            header,
            payload,
            payload_checksum,
        })
    }
}

/// Calculates a simple checksum for the given data
pub fn calculate_checksum(data: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    for &byte in data {
        checksum = checksum.wrapping_add(byte);
    }
    !checksum // Invert bits for better error detection
}