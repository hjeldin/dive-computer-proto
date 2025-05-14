#![no_std]
//! Dive Computer Prototype Library
//!
//! This library provides the core functionality for a dive computer system,
//! including sensor communication, command processing, dive calculations,
//! and communication protocols.
//!
//! # Modules
//!
//! * `sensor` - Defines sensor types and sensor data handling
//! * `commands` - Defines command and response structures for dive computer operations
//! * `dive_calc` - Implements dive-related calculations and algorithms
//! * `protocol` - Provides serialization/deserialization for communication
//! * `examples` - Contains usage examples for the main functionality

/// Sensor types and data handling
pub mod sensor;

/// Command and response structures for dive computer operations
pub mod commands;

/// Dive-related calculations and algorithms
pub mod dive_calc;

/// Serialization/deserialization for communication
pub mod protocol;

/// Usage examples for the main functionality
pub mod examples;
