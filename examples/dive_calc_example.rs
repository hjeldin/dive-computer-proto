use dive_computer_proto::dive_calc::{calculate_ndl, calculate_ppo2, DiveProfile, GasType};

fn main() {
    // Create a dive profile with air
    let mut dive_profile = DiveProfile::new(GasType::Air);
    println!("Created dive profile: {:?}", dive_profile);

    // Update the depth
    dive_profile.update_depth(4000); // 18.3 meters
    println!(
        "Updated depth: {:.1} meters",
        dive_profile.current_depth_cm as f32 / 100.0
    );

    // Increment the dive time
    dive_profile.increment_duration(600); // 10 minutes
    println!(
        "Current dive time: {} seconds",
        dive_profile.duration_seconds
    );

    // Update the temperature
    dive_profile.update_temperature(182); // 18.2°C
    println!(
        "Updated temperature: {:.1}°C",
        dive_profile.temperature_celsius_x10 as f32 / 10.0
    );

    // Calculate no-decompression limit for current depth
    let depth_meters = dive_profile.current_depth_cm / 100;
    let ndl = calculate_ndl(depth_meters, dive_profile.gas_type);
    println!(
        "No-decompression limit at {:.1} meters: {} minutes",
        depth_meters as f32, ndl
    );

    // Calculate PPO2 for current depth
    let ppo2 = calculate_ppo2(depth_meters, dive_profile.gas_type);
    println!(
        "PPO2 at {:.1} meters: {:.2} bar",
        depth_meters as f32,
        ppo2 as f32 / 100.0
    );

    // Example with nitrox
    let nitrox_profile = DiveProfile::new(GasType::Nitrox { oxygen_percent: 32 });
    println!("Created nitrox profile: {:?}", nitrox_profile);

    let nitrox_ndl = calculate_ndl(30, GasType::Nitrox { oxygen_percent: 32 });
    println!(
        "No-decompression limit at 30 meters with 32% nitrox: {} minutes",
        nitrox_ndl
    );
}
