//! Dive calculations and algorithms module
//!
//! This module provides functions and types for performing dive-related calculations
//! such as decompression limits, gas consumption, and other diving metrics.

use dive_deco::{BuhlmannModel, DecoModel, Depth, Gas, Time};
use serde::{Deserialize, Serialize};

/// Gas type used in diving
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum GasType {
    /// Standard air (21% oxygen, 79% nitrogen)
    Air,
    /// Enriched air nitrox with specified oxygen percentage
    Nitrox { oxygen_percent: u8 },
    /// Trimix with specified oxygen, helium, and nitrogen percentages
    Trimix {
        oxygen_percent: u8,
        helium_percent: u8,
    },
}

/// Convert a GasType into a Gas (for use with deco model crate)
impl GasType {
    pub fn into_gas(&self) -> Gas {
        match self {
            GasType::Air => Gas::air(),
            GasType::Nitrox { oxygen_percent } => Gas::new(*oxygen_percent as f64 / 100.0, 0.0),
            GasType::Trimix {
                oxygen_percent,
                helium_percent,
            } => Gas::new(
                *oxygen_percent as f64 / 100.0,
                *helium_percent as f64 / 100.0,
            ),
        }
    }
}

/// Represents a dive profile with depth and time information
#[derive(Serialize, Deserialize, Debug)]
pub struct DiveProfile {
    /// Maximum depth reached during the dive in centimeters
    pub max_depth_cm: u16,
    /// Current depth in centimeters
    pub current_depth_cm: u16,
    /// Dive duration in seconds
    pub duration_seconds: u32,
    /// Gas type being used
    pub gas_type: GasType,
    /// Water temperature in degrees Celsius (scaled by 10, e.g., 215 = 21.5°C)
    pub temperature_celsius_x10: i16,
    /// Ascent rate in cm/minute
    pub ascent_rate_cm_per_min: u16,
    /// Descent rate in cm/minute
    pub descent_rate_cm_per_min: u16,
    /// model
    pub model: BuhlmannModel,
}

impl DiveProfile {
    /// Creates a new dive profile with default values
    pub fn new(gas_type: GasType) -> Self {
        DiveProfile {
            max_depth_cm: 0,
            current_depth_cm: 0,
            duration_seconds: 0,
            gas_type,
            temperature_celsius_x10: 200, // 20.0°C
            ascent_rate_cm_per_min: 0,
            descent_rate_cm_per_min: 0,
            model: BuhlmannModel::default(),
        }
    }

    /// Updates the current depth and recalculates max depth if needed
    pub fn update_depth(&mut self, new_depth_cm: u16) {
        self.current_depth_cm = new_depth_cm;
        if new_depth_cm > self.max_depth_cm {
            self.max_depth_cm = new_depth_cm;
        }
    }

    /// Increments the dive duration by the specified number of seconds
    pub fn increment_duration(&mut self, seconds: u32) {
        self.duration_seconds += seconds;
        // a very simple example of model update, assuming that all changes to dive profile are followed by a duration increment
        let gas = self.gas_type.into_gas();
        self.model.record(
            Depth::from_meters(self.current_depth_cm as f32 / 100.0),
            Time::from_seconds(seconds),
            &gas,
        );
    }

    /// Updates the water temperature
    pub fn update_temperature(&mut self, celsius_x10: i16) {
        self.temperature_celsius_x10 = celsius_x10;
    }
}

/// Calculates the no-decompression limit (NDL) in minutes for a given depth and gas
///
/// This is a simplified static calculation assuming instant travel to the depth and no prior tissue saturation.
pub fn calculate_ndl(depth_meters: u16, gas: GasType) -> u16 {
    let mut model = BuhlmannModel::default();
    let gas = gas.into_gas();
    model.record(Depth::from_meters(depth_meters), Time::zero(), &gas);
    model.ndl().as_minutes() as u16
}

/// Calculates the partial pressure of oxygen (PPO2) for a given depth and gas
///
/// Returns the PPO2 value multiplied by 100 (e.g., 121 = 1.21 bar)
pub fn calculate_ppo2(depth_meters: u16, gas: GasType) -> u16 {
    let ambient_pressure = (depth_meters as f32 / 10.0) + 1.0; // in bar

    let oxygen_fraction = match gas {
        GasType::Air => 0.21,
        GasType::Nitrox { oxygen_percent } => oxygen_percent as f32 / 100.0,
        GasType::Trimix {
            oxygen_percent,
            helium_percent: _,
        } => oxygen_percent as f32 / 100.0,
    };

    (ambient_pressure * oxygen_fraction * 100.0) as u16
}

/// Calculates estimated gas consumption in liters
///
/// * `depth_meters` - Average depth in meters
/// * `duration_minutes` - Dive duration in minutes
/// * `sac_rate` - Surface Air Consumption rate in liters/minute
pub fn calculate_gas_consumption(depth_meters: u16, duration_minutes: u16, sac_rate: f32) -> f32 {
    let ambient_pressure = (depth_meters as f32 / 10.0) + 1.0; // in bar
    sac_rate * ambient_pressure * duration_minutes as f32
}

/// Calculates the equivalent air depth (EAD) for nitrox diving
///
/// Returns the equivalent air depth in meters
pub fn calculate_ead(depth_meters: u16, gas: GasType) -> Option<u16> {
    match gas {
        GasType::Air => Some(depth_meters), // EAD is the same as actual depth for air
        GasType::Nitrox { oxygen_percent } => {
            let nitrogen_fraction = (100 - oxygen_percent) as f32 / 100.0;
            let air_nitrogen_fraction = 0.79;
            let ead =
                ((depth_meters as f32 + 10.0) * nitrogen_fraction / air_nitrogen_fraction) - 10.0;
            Some(ead as u16)
        }
        GasType::Trimix {
            oxygen_percent,
            helium_percent,
        } => {
            let nitrogen_percent = 100 - oxygen_percent - helium_percent;
            let nitrogen_fraction = nitrogen_percent as f32 / 100.0;
            let air_nitrogen_fraction = 0.79;
            let ead =
                ((depth_meters as f32 + 10.0) * nitrogen_fraction / air_nitrogen_fraction) - 10.0;
            Some(ead as u16)
        }
    }
}
