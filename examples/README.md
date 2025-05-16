# Dive Computer Prototype Examples

This directory contains example code demonstrating how to use the different components of the dive computer prototype library.

## Available Examples

1. **Sensor Example** - Demonstrates the creation and use of dive sensors
   - `cargo run --example sensor_example`

2. **Command Example** - Shows how to create and use commands and responses
   - `cargo run --example command_example`

3. **Dive Calculation Example** - Demonstrates dive calculations including NDL, PPO2, etc.
   - `cargo run --example dive_calc_example`

4. **Protocol Example** - Shows how to use the communication protocol for serializing and deserializing messages
   - `cargo run --example protocol_example`

## Running the Examples

To run any example, use the following command from the project root:

```
cargo run --example [example_name]
```

For instance, to run the dive calculation example:

```
cargo run --example dive_calc_example
```

Each example prints out details of what it's doing, making it easier to understand how the library components work.
