use dive_computer_proto::sensor::{ReadingType, Sensor, SensorResponse};

fn main() {
    // Create a new depth sensor
    let depth_sensor = Sensor::new(1, *b"Depth Sensor    ");
    println!("Created depth sensor: {:?}", depth_sensor);

    // Create a sensor response with a depth reading
    let depth_reading = SensorResponse::new(
        1,                  // sensor_id
        ReadingType::Depth, // reading_type
        1520,               // value (15.2 meters)
        1234567890,         // timestamp
    );
    println!("Depth reading: {:?}", depth_reading);

    // Create a temperature sensor
    let temp_sensor = Sensor::new(2, *b"Temp Sensor     ");
    println!("Created temperature sensor: {:?}", temp_sensor);

    // Create a sensor response with a temperature reading
    let temp_reading = SensorResponse::new(
        2,                        // sensor_id
        ReadingType::Temperature, // reading_type
        215,                      // value (21.5°C)
        1234567890,               // timestamp
    );
    println!("Temperature reading: {:?}", temp_reading);
}
