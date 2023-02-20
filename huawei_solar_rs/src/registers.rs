use crate::StringRegister;
use crate::NumericRegister;
use std::marker::PhantomData;

pub const MODEL_NAME: StringRegister = StringRegister { addr: 30000, count: 15 };
pub const SERIAL_NUMBER: StringRegister = StringRegister { addr: 30015, count: 10 };
pub const INPUT_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32064, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const GRID_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32066, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_A_B: NumericRegister<u16> = NumericRegister::<u16> { addr: 32066, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_B_C: NumericRegister<u16> = NumericRegister::<u16> { addr: 32067, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const LINE_VOLTAGE_C_A: NumericRegister<u16> = NumericRegister::<u16> { addr: 32068, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_A_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32069, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_B_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32070, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const PHASE_C_VOLTAGE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32071, count: 1, gain: 10, unit: "V", marker: PhantomData };
pub const GRID_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32072, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_A_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32072, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_B_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32074, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const PHASE_C_CURRENT: NumericRegister<i32> = NumericRegister::<i32> { addr: 32076, count: 2, gain: 1000, unit: "A", marker: PhantomData };
pub const DAY_ACTIVE_POWER_PEAK: NumericRegister<i32> = NumericRegister::<i32> { addr: 32078, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const ACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32080, count: 2, gain: 1, unit: "W", marker: PhantomData };
pub const REACTIVE_POWER: NumericRegister<i32> = NumericRegister::<i32> { addr: 32082, count: 2, gain: 1, unit: "VA", marker: PhantomData };
pub const POWER_FACTOR: NumericRegister<i16> = NumericRegister::<i16> { addr: 32084, count: 1, gain: 1000, unit: "", marker: PhantomData };
pub const GRID_FREQUENCY: NumericRegister<u16> = NumericRegister::<u16> { addr: 32085, count: 1, gain: 100, unit: "Hz", marker: PhantomData };
pub const EFFICIENCY: NumericRegister<u16> = NumericRegister::<u16> { addr: 32086, count: 1, gain: 100, unit: "%", marker: PhantomData };
pub const INTERNAL_TEMPERATURE: NumericRegister<i16> = NumericRegister::<i16> { addr: 32087, count: 1, gain: 10, unit: "°C", marker: PhantomData };
pub const INSULATION_RESISTANCE: NumericRegister<u16> = NumericRegister::<u16> { addr: 32088, count: 1, gain: 100, unit: "MOhm", marker: PhantomData };
pub const DEVICE_STATUS: NumericRegister<u16> = NumericRegister::<u16> { addr: 32089, count: 1, gain: 1, unit: "", marker: PhantomData };
// FAULT_CODE
// TODO: timestamps
pub const STARTUP_TIME: NumericRegister<u32> = NumericRegister::<u32> { addr: 32091, count: 2, gain: 1, unit: "s", marker: PhantomData };
pub const SHUTDOWN_TIME: NumericRegister<u32> = NumericRegister::<u32> { addr: 32093, count: 2, gain: 1, unit: "s", marker: PhantomData };
pub const ACCUMULATED_YIELD_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 32106, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const DAILY_YIELD_ENERGY: NumericRegister<u32> = NumericRegister::<u32> { addr: 32114, count: 2, gain: 100, unit: "kWh", marker: PhantomData };
pub const TIME_ZONE: NumericRegister<i16> = NumericRegister::<i16> { addr: 43006, count: 1, gain: 1, unit: "min", marker: PhantomData };