use crate::logic::params_parse::Variant;
use std::collections::BTreeMap;
use chrono::prelude::*;

fn unpack_numeric(v: &Variant) -> f64 {
    match v {
        Variant::Float(v) => *v,
        _ => 0.0
    }
}

pub fn timestamp_to_str(timestamp: &Variant) -> String {
    let timestamp = unpack_numeric(timestamp) as i64;
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(), Utc).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn print(map: &BTreeMap<String, Variant>) -> String {
    let mut output = String::new();
    output.push_str("---Inverter status---\n");
    for (key, value) in map {
        match key.as_str() {
            "model_name" => output.push_str(&format!("Model name: {}\n", value)),
            "serial_number" => output.push_str(&format!("Serial number: {}\n", value)),
            "input_power" => output.push_str(&format!("Input power: {} W\n", value)),
            "grid_voltage" => output.push_str(&format!("Grid voltage: {} V\n", value)),
            "line_voltage_a_b" => output.push_str(&format!("Line voltage A-B: {} V\n", value)),
            "line_voltage_b_c" => output.push_str(&format!("Line voltage B-C: {} V\n", value)),
            "line_voltage_c_a" => output.push_str(&format!("Line voltage C-A: {} V\n", value)),
            "phase_a_voltage" => output.push_str(&format!("Phase A voltage: {} V\n", value)),
            "phase_b_voltage" => output.push_str(&format!("Phase B voltage: {} V\n", value)),
            "phase_c_voltage" => output.push_str(&format!("Phase C voltage: {} V\n", value)),
            "phase_a_current" => output.push_str(&format!("Phase A current: {} A\n", value)),
            "phase_b_current" => output.push_str(&format!("Phase B current: {} A\n", value)),
            "phase_c_current" => output.push_str(&format!("Phase C current: {} A\n", value)),
            "day_active_power_peak" => output.push_str(&format!("Day active power peak: {} W\n", value)),
            "active_power" => output.push_str(&format!("Active power: {} W\n", value)),
            "reactive_power" => output.push_str(&format!("Reactive power: {} VA\n", value)),
            "power_factor" => output.push_str(&format!("Power factor: {}\n", value)),
            "grid_frequency" => output.push_str(&format!("Grid frequency: {} Hz\n", value)),
            "efficiency" => output.push_str(&format!("Efficiency: {} %\n", value)),
            "internal_temperature" => output.push_str(&format!("Internal temperature: {} °C\n", value)),
            "insulation_resistance" => output.push_str(&format!("Insulation resistance: {} MΩ\n", value)),
            "device_status" => output.push_str(&format!("Device status: {}\n", value)),
            "startup_time" => output.push_str(&format!("Startup time: {} (inverter's time)\n", timestamp_to_str(value))),
            "shutdown_time" => output.push_str(&format!("Shutdown time: {} (inverter's time)\n", timestamp_to_str(value))),
            "accumulated_yield_energy" => output.push_str(&format!("Accumulated yield energy: {} kWh\n", value)),
            "daily_yield_energy" => output.push_str(&format!("Daily yield energy: {} kWh\n", value)),
            &_ => {}
        }
    }
    output
}
                