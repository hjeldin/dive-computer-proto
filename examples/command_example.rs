use dive_computer_proto::commands::{Command, Response, ResponsePayload};
use dive_computer_proto::sensor::ReadingType;

fn main() {
    // Create a command to request device identification
    let id_command = Command::ID;
    println!("ID command: {:?}", id_command);

    // Create a command to read from a depth sensor
    let read_depth_command = Command::ReadSensor {
        sensor_id: 1,
        reading_type: ReadingType::Depth as u8,
    };
    println!("Read depth command: {:?}", read_depth_command);

    // Create a command to start a dive
    let start_dive_command = Command::StartDive;
    println!("Start dive command: {:?}", start_dive_command);

    // Create a command to set dive parameters
    let set_params_command = Command::SetParameters {
        max_depth: 30, // 30 meters
        max_time: 45,  // 45 minutes
    };
    println!("Set parameters command: {:?}", set_params_command);

    // Example of creating a response to the ID command
    let id_response = Response::success(
        1,          // response id
        1,          // command id
        1234567890, // timestamp
        Some(ResponsePayload::DeviceInfo {
            device_id: 12345,
            firmware_version: [1, 0, 0, 0],
            hardware_version: [1, 0, 0, 0],
        }),
    );
    println!("ID response: {:?}", id_response);

    // Example of creating an error response
    let error_response = Response::error(
        2,          // response id
        2,          // command id
        1234567890, // timestamp
        101,        // error code
    );
    println!("Error response: {:?}", error_response);
}
