use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Sensor {
    id: u8,
    name: [u8; 16],
}

impl Sensor {
    pub fn new() -> Self {
        Sensor {
            id: 0,
            name: [0; 16],
        }
    }
}

#[derive(Debug)]
pub struct SensorResponse {

}