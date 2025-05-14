//! Sensor module for dive computer
//!
//! This module defines the structures and functionality for working with
//! various sensors in a dive computer system, including sensor identification,
//! configuration, and data handling.

use serde::{Serialize, Deserialize};

/// Represents a physical sensor device connected to the dive computer
///
/// Each sensor has a unique ID and a name to identify it in the system.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Sensor {
    /// Unique identifier for the sensor
    pub id: u16,
    /// Name of the sensor (fixed-length array for no_std compatibility)
    pub name: [u8; 16],
}

impl Sensor {
    /// Creates a new sensor with the specified ID and name
    ///
    /// # Arguments
    ///
    /// * `sensor_id` - Unique identifier for the sensor
    /// * `name` - Name of the sensor as a fixed-length byte array
    ///
    /// # Returns
    ///
    /// A new `Sensor` instance
    pub fn new(sensor_id: u16, name: [u8; 16]) -> Self {
        Sensor {
            id: sensor_id,
            name,
        }
    }
}

/// Represents a response from a sensor containing a reading value
///
/// This struct encapsulates data received from a sensor, including the
/// type of reading, its value, and when it was taken.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SensorResponse {
    /// ID of the sensor that provided this reading
    pub sensor_id: u16,
    /// Type of reading (depth, temperature, etc.)
    pub reading_type: ReadingType,
    /// Value of the reading (units depend on reading_type)
    pub value: i32,
    /// Timestamp when the reading was taken (milliseconds since system start)
    pub timestamp: u64,
}

impl SensorResponse {
    /// Creates a new sensor response with the specified parameters
    ///
    /// # Arguments
    ///
    /// * `sensor_id` - ID of the sensor that provided this reading
    /// * `reading_type` - Type of reading (depth, temperature, etc.)
    /// * `value` - Value of the reading (units depend on reading_type)
    /// * `timestamp` - Timestamp when the reading was taken
    ///
    /// # Returns
    ///
    /// A new `SensorResponse` instance
    pub fn new(sensor_id: u16, reading_type: ReadingType, value: i32, timestamp: u64) -> Self {
        SensorResponse {
            sensor_id,
            reading_type,
            value,
            timestamp,
        }
    }
}

/// Types of readings that can be provided by sensors
///
/// Each variant represents a different physical quantity that can be measured.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum ReadingType {
    /// Water depth in centimeters
    Depth,
    /// Temperature in degrees Celsius (scaled by 10, e.g., 215 = 21.5°C)
    Temperature,
    /// Pressure in millibars
    Pressure,
    /// Battery level in percentage (0-100)
    Battery,
}
