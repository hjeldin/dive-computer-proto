//! Usage examples for the dive computer prototype
//!
//! This module contains examples demonstrating how to use the main functionality
//! of the dive computer prototype library.

use crate::sensor::{Sensor, SensorResponse, ReadingType};
use crate::commands::{Command, Response, ResponseStatus, ResponsePayload, MessageType};
use crate::dive_calc::{GasType, DiveProfile, calculate_ndl, calculate_ppo2};
use crate::protocol::{Message, MessageKind, ProtocolError};

/// Example of creating and using sensors
pub fn sensor_example() {
    // Create a new depth sensor
    let depth_sensor = Sensor::new(1, *b"Depth Sensor    ");
    
    // Create a sensor response with a depth reading
    let depth_reading = SensorResponse::new(
        1,                      // sensor_id
        ReadingType::Depth,     // reading_type
        1520,                   // value (15.2 meters)
        1234567890,             // timestamp
    );
    
    // Create a temperature sensor
    let temp_sensor = Sensor::new(2, *b"Temp Sensor     ");
    
    // Create a sensor response with a temperature reading
    let temp_reading = SensorResponse::new(
        2,                      // sensor_id
        ReadingType::Temperature, // reading_type
        215,                    // value (21.5°C)
        1234567890,             // timestamp
    );
    
    // Process the readings (in a real application)
    // ...
}

/// Example of sending commands and processing responses
pub fn command_example() {
    // Create a command to request device identification
    let id_command = Command::ID;
    
    // Create a command to read from a depth sensor
    let read_depth_command = Command::ReadSensor {
        sensor_id: 1,
        reading_type: ReadingType::Depth as u8,
    };
    
    // Create a command to start a dive
    let start_dive_command = Command::StartDive;
    
    // Create a command to set dive parameters
    let set_params_command = Command::SetParameters {
        max_depth: 30,  // 30 meters
        max_time: 45,   // 45 minutes
    };
    
    // Example of creating a response to the ID command
    let id_response = Response::success(
        1,              // response id
        1,              // command id
        1234567890,     // timestamp
        Some(ResponsePayload::DeviceInfo {
            device_id: 12345,
            firmware_version: [1, 0, 0, 0],
            hardware_version: [1, 0, 0, 0],
        }),
    );
    
    // Example of creating an error response
    let error_response = Response::error(
        2,              // response id
        2,              // command id
        1234567890,     // timestamp
        101,            // error code
    );
    
    // Process the responses (in a real application)
    // ...
}

/// Example of using dive calculations
pub fn dive_calc_example() {
    // Create a dive profile with air
    let mut dive_profile = DiveProfile::new(GasType::Air);
    
    // Update the depth
    dive_profile.update_depth(1830);  // 18.3 meters
    
    // Increment the dive time
    dive_profile.increment_duration(600);  // 10 minutes
    
    // Update the temperature
    dive_profile.update_temperature(182);  // 18.2°C
    
    // Calculate no-decompression limit for current depth
    let depth_meters = dive_profile.current_depth_cm / 100;
    let ndl = calculate_ndl(depth_meters as u16, dive_profile.gas);
    
    // Calculate PPO2 for current depth
    let ppo2 = calculate_ppo2(depth_meters as u16, dive_profile.gas);
    
    // Example with nitrox
    let nitrox_profile = DiveProfile::new(GasType::Nitrox { oxygen_percent: 32 });
    let nitrox_ndl = calculate_ndl(18, GasType::Nitrox { oxygen_percent: 32 });
    
    // Process the calculations (in a real application)
    // ...
}

/// Example of using the communication protocol
pub fn protocol_example() -> Result<(), ProtocolError> {
    // Create a command message
    let command = Command::GetBatteryStatus;
    let command_message = Message::new(
        MessageKind::Command,
        123,  // sequence number
        command,
    )?;
    
    // Serialize the message
    let (size, buffer) = command_message.serialize()?;
    
    // In a real application, the buffer would be sent to the device
    // ...
    
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
        123,  // sequence number (matching the request)
        response,
    )?;
    
    // Serialize the response
    let (response_size, response_buffer) = response_message.serialize()?;
    
    // In a real application, the response buffer would be sent back
    // ...
    
    Ok(())
}