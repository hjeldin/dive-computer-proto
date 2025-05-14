# Dive Computer Binary Protocol Format

This document describes the binary protocol format used for communication between the dive computer and sensors or external devices.

## Overview

The dive computer uses a binary protocol for efficient communication with minimal overhead, suitable for embedded systems with limited resources. The protocol is designed to be robust, with checksums for data integrity verification and a consistent message structure.

## Message Structure

Each message consists of three parts:

1. **Header**: Contains metadata about the message
2. **Payload**: Contains the actual data being transmitted
3. **Checksum**: A simple checksum for the payload to verify data integrity

### Header Format

The header is structured as follows:

| Field           | Size (bytes) | Description                                       |
|-----------------|--------------|---------------------------------------------------|
| Magic Bytes     | 2            | Fixed value `[0xDC, 0x42]` to identify valid messages |
| Version         | 1            | Protocol version (current: 1)                     |
| Message Kind    | 1            | Type of message (Command, Response, etc.)         |
| Sequence Number | 2            | Used to match requests with responses             |
| Payload Length  | 2            | Length of the payload in bytes                    |
| Header Checksum | 1            | Checksum of the header fields                     |

Total header size: 9 bytes

### Message Kinds

The following message kinds are defined:

| Value | Kind         | Description                                     |
|-------|-------------|-------------------------------------------------|
| 0x01  | Command     | Command message sent to the dive computer       |
| 0x02  | Response    | Response message sent from the dive computer    |
| 0x03  | Notification| Unsolicited message sent from the dive computer |
| 0x04  | Ack         | Simple acknowledgment                           |
| 0x05  | Error       | Error message                                   |

### Payload Format

The payload format depends on the message kind and the specific command or response. The payload is serialized using the [Postcard](https://github.com/jamesmunns/postcard) format, which is a compact binary serialization format for Rust's Serde.

### Checksum Calculation

The checksum is calculated by summing all bytes in the data and then inverting the bits. This provides a simple but effective way to detect transmission errors.

```rust
fn calculate_checksum(data: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    for &byte in data {
        checksum = checksum.wrapping_add(byte);
    }
    !checksum // Invert bits for better error detection
}
```

## Sensor Communication

### Sensor Reading Request

To request a reading from a sensor, the dive computer sends a `Command::ReadSensor` message:

```
Header:
  - Magic Bytes: [0xDC, 0x42]
  - Version: 0x01
  - Message Kind: 0x01 (Command)
  - Sequence Number: <unique id>
  - Payload Length: <length of serialized Command::ReadSensor>
  - Header Checksum: <calculated>

Payload:
  - Serialized Command::ReadSensor { sensor_id, reading_type }

Payload Checksum:
  - <calculated>
```

### Sensor Reading Response

The sensor responds with a `Response` message containing a `SensorData` payload:

```
Header:
  - Magic Bytes: [0xDC, 0x42]
  - Version: 0x01
  - Message Kind: 0x02 (Response)
  - Sequence Number: <matching the request>
  - Payload Length: <length of serialized Response>
  - Header Checksum: <calculated>

Payload:
  - Serialized Response with ResponsePayload::SensorData { sensor_id, reading_type, value }

Payload Checksum:
  - <calculated>
```

## Error Handling

If an error occurs during communication, an error response is sent with an appropriate error code:

```
Header:
  - Magic Bytes: [0xDC, 0x42]
  - Version: 0x01
  - Message Kind: 0x05 (Error)
  - Sequence Number: <matching the request, if applicable>
  - Payload Length: <length of serialized error information>
  - Header Checksum: <calculated>

Payload:
  - Serialized Response with ResponseStatus::Error and ResponsePayload::ErrorInfo { code }

Payload Checksum:
  - <calculated>
```

## Common Error Codes

| Code | Description                     |
|------|---------------------------------|
| 0x01 | Invalid command                 |
| 0x02 | Sensor not found                |
| 0x03 | Reading type not supported      |
| 0x04 | Sensor communication failure    |
| 0x05 | Device busy                     |
| 0x06 | Invalid parameters              |
| 0x07 | Timeout                         |
| 0x08 | Insufficient permissions        |
| 0x09 | Low battery                     |
| 0x0A | Internal error                  |

## Example Message Flow

### Example 1: Reading Depth

1. Dive computer sends a `ReadSensor` command for the depth sensor
2. Sensor responds with a `Response` containing the depth reading

### Example 2: Setting Dive Parameters

1. Dive computer receives a `SetParameters` command from an external device
2. Dive computer applies the parameters and sends a `Response` with status `Success`

### Example 3: Error Handling

1. Dive computer sends a `ReadSensor` command for a non-existent sensor
2. A `Response` with status `Error` and error code `0x02` (Sensor not found) is returned