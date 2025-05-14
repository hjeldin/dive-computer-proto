//! Command and response handling for dive computer operations
//!
//! This module defines the command and response structures used for
//! communication between the dive computer and external devices or systems.
//! It includes command types, response formats, and status codes.

use serde::{Serialize, Deserialize};

/// Types of messages that can be exchanged in the dive computer system
///
/// This enum distinguishes between command messages (sent to the dive computer)
/// and response messages (sent from the dive computer).
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    /// Command message sent to the dive computer
    Command,
    /// Response message sent from the dive computer
    Response,
}

/// Commands that can be sent to the dive computer
///
/// This enum represents all possible operations that can be requested
/// from the dive computer system.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Request device identification
    ID,
    /// Read data from a specific sensor
    /// 
    /// * `sensor_id` - ID of the sensor to read from
    /// * `reading_type` - Type of reading to request (see ReadingType enum)
    ReadSensor { sensor_id: u16, reading_type: u8 },
    /// Start a new dive session
    StartDive,
    /// End the current dive session
    EndDive,
    /// Set dive parameters for the current or next dive
    /// 
    /// * `max_depth` - Maximum depth in meters
    /// * `max_time` - Maximum dive time in minutes
    SetParameters { max_depth: u16, max_time: u16 },
    /// Get current dive parameters
    GetParameters,
    /// Store a dive log entry
    /// 
    /// * `dive_id` - Unique identifier for the dive
    /// * `data` - Serialized dive data
    LogDive { dive_id: u32, data: [u8; 32] },
    /// Retrieve a dive log entry
    /// 
    /// * `dive_id` - Unique identifier for the dive to retrieve
    GetDiveLog { dive_id: u32 },
    /// Get battery status information
    GetBatteryStatus,
    /// Enter low power mode to conserve battery
    EnterLowPowerMode,
    /// Exit low power mode
    ExitLowPowerMode,
    /// Calibrate sensors to ensure accurate readings
    CalibrateSensors,
    /// Run self-diagnostic to check system health
    RunDiagnostic,
    /// Reset device to factory settings (clears all data)
    FactoryReset,
    /// Start firmware update process
    /// 
    /// * `version` - Version number of the new firmware
    /// * `total_chunks` - Total number of data chunks to be sent
    UpdateFirmwareStart { version: [u8; 4], total_chunks: u16 },
    /// Send a firmware data chunk
    /// 
    /// * `chunk_id` - Sequential ID of this chunk
    /// * `data` - Binary firmware data
    UpdateFirmwareChunk { chunk_id: u16, data: [u8; 32] },
    /// Complete firmware update process
    UpdateFirmwareComplete
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    /// Unique response ID
    pub id: u32,
    /// ID of the command that triggered this response
    pub command_id: u32,
    /// Status code of the response
    pub status: ResponseStatus,
    /// Timestamp when the response was generated
    pub timestamp: u64,
    /// Optional payload data
    pub payload: Option<ResponsePayload>,
}

impl Response {
    /// Creates a new response with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this response
    /// * `command_id` - ID of the command that triggered this response
    /// * `status` - Status code indicating the result of the command
    /// * `timestamp` - Time when the response was generated
    /// * `payload` - Optional data payload containing response-specific information
    ///
    /// # Returns
    ///
    /// A new `Response` instance
    pub fn new(id: u32, command_id: u32, status: ResponseStatus, timestamp: u64, payload: Option<ResponsePayload>) -> Self {
        Response {
            id,
            command_id,
            status,
            timestamp,
            payload,
        }
    }

    /// Creates a success response with the specified parameters
    ///
    /// This is a convenience method for creating responses with a Success status.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this response
    /// * `command_id` - ID of the command that triggered this response
    /// * `timestamp` - Time when the response was generated
    /// * `payload` - Optional data payload containing response-specific information
    ///
    /// # Returns
    ///
    /// A new `Response` instance with Success status
    pub fn success(id: u32, command_id: u32, timestamp: u64, payload: Option<ResponsePayload>) -> Self {
        Response::new(id, command_id, ResponseStatus::Success, timestamp, payload)
    }

    /// Creates an error response with the specified parameters
    ///
    /// This is a convenience method for creating responses with an Error status
    /// and an ErrorInfo payload containing the error code.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this response
    /// * `command_id` - ID of the command that triggered this response
    /// * `timestamp` - Time when the response was generated
    /// * `error_code` - Code identifying the specific error that occurred
    ///
    /// # Returns
    ///
    /// A new `Response` instance with Error status and ErrorInfo payload
    pub fn error(id: u32, command_id: u32, timestamp: u64, error_code: u16) -> Self {
        Response::new(
            id, 
            command_id, 
            ResponseStatus::Error, 
            timestamp, 
            Some(ResponsePayload::ErrorInfo { code: error_code })
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum ResponseStatus {
    /// Command executed successfully
    Success,
    /// Command execution failed
    Error,
    /// Command is being processed
    InProgress,
    /// Command was received but execution is delayed
    Pending,
}

/// Payload data for responses
///
/// This enum represents the different types of data that can be included
/// in a response from the dive computer.
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponsePayload {
    /// Device identification information
    DeviceInfo {
        /// Unique identifier for the device
        device_id: u32,
        /// Firmware version as [major, minor, patch, build]
        firmware_version: [u8; 4],
        /// Hardware version as [major, minor, patch, revision]
        hardware_version: [u8; 4],
    },
    /// Sensor reading data
    SensorData {
        /// ID of the sensor that provided the reading
        sensor_id: u8,
        /// Type of reading (depth, temperature, etc.)
        reading_type: u8,
        /// Value of the reading (units depend on reading_type)
        value: i32,
    },
    /// Current dive parameters
    DiveParameters {
        /// Maximum depth setting in centimeters
        max_depth: u16,
        /// Maximum dive time setting in minutes
        max_time: u16,
        /// Current depth in centimeters
        current_depth: u16,
        /// Elapsed dive time in seconds
        elapsed_time: u16,
    },
    /// Dive log entry
    DiveLog {
        /// Unique identifier for the dive
        dive_id: u32,
        /// Serialized dive data
        data: [u8; 32],
    },
    /// Battery status information
    BatteryStatus {
        /// Battery level as percentage (0-100)
        level: u8,
        /// Battery voltage in millivolts
        voltage: u16,
        /// Estimated time remaining in minutes
        estimated_time_remaining: u16,
    },
    /// Diagnostic results
    DiagnosticResults {
        /// Overall status code (0 = all good, non-zero = issues detected)
        status: u8,
        /// Specific error codes for different subsystems
        error_codes: [u8; 4],
    },
    /// Error information
    ErrorInfo {
        /// Specific error code
        code: u16,
    },
    /// Acknowledgment with no data
    Ack,
}
