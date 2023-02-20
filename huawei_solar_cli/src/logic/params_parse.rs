use huawei_solar_rs::HuaweiSolar;
use huawei_solar_rs::NumericRegisterTrait;
use huawei_solar_rs::StringRegister;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use huawei_solar_rs::registers;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Variant {
    String(String),
    Float(f64)
}

fn unpack_numeric(v: &Variant) -> f64 {
    match v {
        Variant::Float(v) => *v,
        _ => 0.0
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variant::String(s) => write!(f, "{}", s),
            Variant::Float(flt) => write!(f, "{}", flt)
        }
    }
}

fn handle_string_reg(inv: &mut HuaweiSolar, name: &str, reg: &StringRegister, map: &mut BTreeMap<String, Variant>) {
    let value = match inv.read_string_register(reg) {
        Ok(value) => value,
        Err(e) => {
            println!("Error reading {}: {}", name, e);
            return;
        }
    };
    map.insert(name.to_string(), Variant::String(value));
}

fn handle_numeric_reg<T: NumericRegisterTrait>(inv: &mut HuaweiSolar, name: &str, reg: &T, map: &mut BTreeMap<String, Variant>) {
    let value = match inv.read_numeric_register(reg) {
        Ok(value) => value,
        Err(e) => {
            println!("Error reading {}: {}", name, e);
            return;
        }
    };
    map.insert(name.to_string(), Variant::Float(value));
}

fn handle_device_status(inv: &mut HuaweiSolar, map: &mut BTreeMap<String, Variant>) {
    let value = match inv.read_device_status(&registers::DEVICE_STATUS) {
        Ok(value) => value,
        Err(e) => {
            println!("Error reading device status: {}", e);
            return;
        }
    };
    map.insert("device_status".to_string(), Variant::String(value));
}

fn handle_timestamp_reg<T: NumericRegisterTrait>(inv: &mut HuaweiSolar, name: &str, reg: &T, map: &mut BTreeMap<String, Variant>) {
    if !map.contains_key("time_zone") {
        handle_numeric_reg(inv, "time_zone", &registers::TIME_ZONE, map);
    }
    let value = match inv.read_numeric_register(reg) {
        Ok(value) => value,
        Err(e) => {
            println!("Error reading {}: {}", name, e);
            return;
        }
    };
    map.insert(name.to_string(), Variant::Float(value - 60.0 * unpack_numeric(map.get("time_zone").unwrap())));
}

pub fn parse_to_map(inverter: &mut HuaweiSolar, query_params: &str) -> BTreeMap<String, Variant> {
    let mut map = BTreeMap::new();
    if query_params.contains("all") {
        handle_string_reg(inverter, "model_name", &registers::MODEL_NAME, &mut map);
        handle_string_reg(inverter, "serial_number", &registers::SERIAL_NUMBER, &mut map);
        handle_numeric_reg(inverter, "input_power", &registers::INPUT_POWER, &mut map);
        handle_numeric_reg(inverter, "grid_voltage", &registers::GRID_VOLTAGE, &mut map);
        handle_numeric_reg(inverter, "line_voltage_a_b", &registers::LINE_VOLTAGE_A_B, &mut map);
        handle_numeric_reg(inverter, "line_voltage_b_c", &registers::LINE_VOLTAGE_B_C, &mut map);
        handle_numeric_reg(inverter, "line_voltage_c_a", &registers::LINE_VOLTAGE_C_A, &mut map);
        handle_numeric_reg(inverter, "phase_a_voltage", &registers::PHASE_A_VOLTAGE, &mut map);
        handle_numeric_reg(inverter, "phase_b_voltage", &registers::PHASE_B_VOLTAGE, &mut map);
        handle_numeric_reg(inverter, "phase_c_voltage", &registers::PHASE_C_VOLTAGE, &mut map);
        handle_numeric_reg(inverter, "grid_current", &registers::GRID_CURRENT, &mut map);
        handle_numeric_reg(inverter, "phase_a_current", &registers::PHASE_A_CURRENT, &mut map);
        handle_numeric_reg(inverter, "phase_b_current", &registers::PHASE_B_CURRENT, &mut map);
        handle_numeric_reg(inverter, "phase_c_current", &registers::PHASE_C_CURRENT, &mut map);
        handle_numeric_reg(inverter, "day_active_power_peak", &registers::DAY_ACTIVE_POWER_PEAK, &mut map);
        handle_numeric_reg(inverter, "active_power", &registers::ACTIVE_POWER, &mut map);
        handle_numeric_reg(inverter, "reactive_power", &registers::REACTIVE_POWER, &mut map);
        handle_numeric_reg(inverter, "power_factor", &registers::POWER_FACTOR, &mut map);
        handle_numeric_reg(inverter, "grid_frequency", &registers::GRID_FREQUENCY, &mut map);
        handle_numeric_reg(inverter, "efficiency", &registers::EFFICIENCY, &mut map);
        handle_numeric_reg(inverter, "internal_temperature", &registers::INTERNAL_TEMPERATURE, &mut map);
        handle_numeric_reg(inverter, "insulation_resistance", &registers::INSULATION_RESISTANCE, &mut map);
        handle_device_status(inverter, &mut map);
        handle_timestamp_reg(inverter, "startup_time", &registers::STARTUP_TIME, &mut map);
        handle_timestamp_reg(inverter, "shutdown_time", &registers::SHUTDOWN_TIME, &mut map);
        handle_numeric_reg(inverter, "accumulated_yield_energy", &registers::ACCUMULATED_YIELD_ENERGY, &mut map);
        handle_numeric_reg(inverter, "daily_yield_energy", &registers::DAILY_YIELD_ENERGY, &mut map);

    } else {
        for param in query_params.split(",") {
            match param {
                "model_name" => {
                    handle_string_reg(inverter, "model_name", &registers::MODEL_NAME, &mut map)
                },
                "serial_number" => {
                    handle_string_reg(inverter, "serial_number", &registers::SERIAL_NUMBER, &mut map)
                },
                "input_power" => {
                    handle_numeric_reg(inverter, "input_power", &registers::INPUT_POWER, &mut map)
                },
                "grid_voltage" => {
                    handle_numeric_reg(inverter, "grid_voltage", &registers::GRID_VOLTAGE, &mut map)
                },
                "line_voltage_a_b" => {
                    handle_numeric_reg(inverter, "line_voltage_a_b", &registers::LINE_VOLTAGE_A_B, &mut map)
                },
                "line_voltage_b_c" => {
                    handle_numeric_reg(inverter, "line_voltage_b_c", &registers::LINE_VOLTAGE_B_C, &mut map)
                },
                "line_voltage_c_a" => {
                    handle_numeric_reg(inverter, "line_voltage_c_a", &registers::LINE_VOLTAGE_C_A, &mut map)
                },
                "phase_a_voltage" => {
                    handle_numeric_reg(inverter, "phase_a_voltage", &registers::PHASE_A_VOLTAGE, &mut map)
                },
                "phase_b_voltage" => {
                    handle_numeric_reg(inverter, "phase_b_voltage", &registers::PHASE_B_VOLTAGE, &mut map)
                },
                "phase_c_voltage" => {
                    handle_numeric_reg(inverter, "phase_c_voltage", &registers::PHASE_C_VOLTAGE, &mut map)
                },
                "grid_current" => {
                    handle_numeric_reg(inverter, "grid_current", &registers::GRID_CURRENT, &mut map)
                },
                "phase_a_current" => {
                    handle_numeric_reg(inverter, "phase_a_current", &registers::PHASE_A_CURRENT, &mut map)
                },
                "phase_b_current" => {
                    handle_numeric_reg(inverter, "phase_b_current", &registers::PHASE_B_CURRENT, &mut map)
                },
                "phase_c_current" => {
                    handle_numeric_reg(inverter, "phase_c_current", &registers::PHASE_C_CURRENT, &mut map)
                },
                "day_active_power_peak" => {
                    handle_numeric_reg(inverter, "day_active_power_peak", &registers::DAY_ACTIVE_POWER_PEAK, &mut map)
                },
                "active_power" => {
                    handle_numeric_reg(inverter, "active_power", &registers::ACTIVE_POWER, &mut map)
                },
                "reactive_power" => {
                    handle_numeric_reg(inverter, "reactive_power", &registers::REACTIVE_POWER, &mut map)
                },
                "power_factor" => {
                    handle_numeric_reg(inverter, "power_factor", &registers::POWER_FACTOR, &mut map)
                },
                "grid_frequency" => {
                    handle_numeric_reg(inverter, "grid_frequency", &registers::GRID_FREQUENCY, &mut map)
                },
                "efficiency" => {
                    handle_numeric_reg(inverter, "efficiency", &registers::EFFICIENCY, &mut map)
                },
                "internal_temperature" => {
                    handle_numeric_reg(inverter, "internal_temperature", &registers::INTERNAL_TEMPERATURE, &mut map)
                },
                "insulation_resistance" => {
                    handle_numeric_reg(inverter, "insulation_resistance", &registers::INSULATION_RESISTANCE, &mut map)
                },
                "device_status" => {
                    handle_device_status(inverter, &mut map)
                },
                "startup_time" => {
                    handle_timestamp_reg(inverter, "startup_time", &registers::STARTUP_TIME, &mut map)
                },
                "shutdown_time" => {
                    handle_timestamp_reg(inverter, "shutdown_time", &registers::SHUTDOWN_TIME, &mut map)
                },
                "accumulated_yield_energy" => {
                    handle_numeric_reg(inverter, "accumulated_yield_energy", &registers::ACCUMULATED_YIELD_ENERGY, &mut map)
                },
                "daily_yield_energy" => {
                    handle_numeric_reg(inverter, "daily_yield_energy", &registers::DAILY_YIELD_ENERGY, &mut map)
                },
                _ => println!("Unknown parameter: {}", param),
            }
        }
    }
    
    map
}