use dive_computer_proto::commands::{Command, Response, ResponsePayload};
use dive_computer_proto::protocol::{Message, MessageKind, ProtocolError};

fn main() -> Result<(), ProtocolError> {
    // Create a command message
    let command = Command::GetBatteryStatus;
    let command_message = Message::new(
        MessageKind::Command,
        123, // sequence number
        command,
    )?;
    println!("Created command message: {:?}", command_message);

    // Serialize the message
    let (size, buffer) = command_message.serialize()?;
    println!("Serialized message size: {} bytes", size);
    println!(
        "First few bytes of the serialized message: {:?}",
        &buffer[..size.min(10)]
    );

    // In a real application, the buffer would be sent to the device
    println!("In a real application, this buffer would be sent to the device...");

    // Example of creating a response message
    let response_payload = ResponsePayload::BatteryStatus {
        level: 85,
        voltage: 3720,
        estimated_time_remaining: 120,
    };

    let response = Response::success(
        456,        // response id
        123,        // command id (matching the request)
        1234567890, // timestamp
        Some(response_payload),
    );

    let response_message = Message::new(
        MessageKind::Response,
        123, // sequence number (matching the request)
        response,
    )?;
    println!("Created response message: {:?}", response_message);

    // Serialize the response
    let (response_size, response_buffer) = response_message.serialize()?;
    println!("Serialized response size: {} bytes", response_size);
    println!(
        "First few bytes of the serialized response: {:?}",
        &response_buffer[..response_size.min(10)]
    );

    println!("In a real application, this response would be processed by the client...");

    Ok(())
}
