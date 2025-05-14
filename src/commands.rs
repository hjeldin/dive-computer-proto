use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Command,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    ID
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: u32,
    source_id: u32,
}