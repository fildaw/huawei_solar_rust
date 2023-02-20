use modbus::{Client};
use modbus::tcp;
use std::marker::PhantomData;
use std::{thread, time, str};
use byteorder::{WriteBytesExt, BigEndian};

pub mod registers;

pub trait NumericRegisterTrait {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error>;
}

pub struct NumericRegister<T> {
    addr: u16,
    count: u16,
    gain: u32,
    unit: &'static str,
    marker: PhantomData<T>
}

impl<T> NumericRegister<T> {
    pub fn new(addr: u16, count: u16, gain: u32, unit: &'static str) -> NumericRegister<T> {
        NumericRegister {
            addr: addr,
            count: count,
            gain: gain,
            unit: unit,
            marker: PhantomData
        }
    }

    pub fn get_unit(&self) -> &'static str {
        self.unit
    }
}

impl NumericRegisterTrait for NumericRegister<u16> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut result: Vec<f64> = Vec::new();
        for elem in resp {
            result.push((elem as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<u32> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..(bytes.len() / 4) {
            result.push((u32::from_be_bytes([bytes[i * 4], bytes[i * 4 + 1], bytes[i * 4 + 2], bytes[i * 4 + 3]]) as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<i16> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut result: Vec<f64> = Vec::new();
        for elem in resp {
            result.push((elem as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

impl NumericRegisterTrait for NumericRegister<i32> {
    fn read(&self, client: &mut modbus::tcp::Transport) -> Result<Vec<f64>, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..(bytes.len() / 4) {
            result.push((i32::from_be_bytes([bytes[i * 4], bytes[i * 4 + 1], bytes[i * 4 + 2], bytes[i * 4 + 3]]) as f64) / (self.gain as f64));
        }
        Ok(result)
    }
}

pub struct StringRegister {
    addr: u16,
    count: u16
}

impl StringRegister {
    pub fn new(addr: u16, count: u16) -> StringRegister {
        StringRegister {
            addr: addr,
            count: count
        }
    }

    pub fn read(&self, client: &mut modbus::tcp::Transport) -> Result<String, modbus::Error> {
        let resp = client.read_holding_registers(self.addr, self.count)?;
        let mut bytes: Vec<u8> = Vec::new();
        for elem in resp {
            bytes.write_u16::<BigEndian>(elem).unwrap();
        }
        Ok(str::from_utf8(&bytes).unwrap().replace(char::from(0), "").to_string())
    }
}

pub const DEVICE_STATUS_DEFINITIONS: [(u16, &'static str); 30] = [
    (0x0000, "Standby, initializing"),
    (0x0001, "Standby, detecting insulation resistance"),
    (0x0002, "Standby, detecting irradiation"),
    (0x0003, "Standby, grid detecting"),
    (0x0100, "Starting"),
    (0x0200, "On-grid"),
    (0x0201, "Grid Connection, power limited"),
    (0x0202, "Grid Connection, self-derating"),
    (0x0300, "Shutdown, fault"),
    (0x0301, "Shutdown, command"),
    (0x0302, "Shutdown, OVGR"),
    (0x0303, "Shutdown, communication disconnected"),
    (0x0304, "Shutdown, power limited"),
    (0x0305, "Shutdown, manual startup required"),
    (0x0306, "Shutdown, DC switches disconnected"),
    (0x0307, "Shutdown, rapid cutoff"),
    (0x0308, "Shutdown, input underpowered"),
    (0x0401, "Grid scheduling, cosphi-P curve"),
    (0x0402, "Grid scheduling, Q-U curve"),
    (0x0403, "Grid scheduling, PF-U curve"),
    (0x0404, "Grid scheduling, dry contact"),
    (0x0405, "Grid scheduling, Q-P curve"),
    (0x0500, "Spot-check ready"),
    (0x0501, "Spot-checking"),
    (0x0600, "Inspecting"),
    (0x0700, "AFCI self check"),
    (0x0800, "I-V scanning"),
    (0x0900, "DC input detection"),
    (0x0A00, "Running, off-grid charging"),
    (0xA000, "Standby, no irradiation"),
];

pub struct HuaweiSolar {
    pub client: modbus::tcp::Transport
}

impl HuaweiSolar {
    #[allow(unused_mut)]
    pub fn new_connection(ip: &str, port: u16, slave_id: u8) -> Result<HuaweiSolar, modbus::Error> {
        let mut cfg = tcp::Config::default();
        cfg.modbus_uid = slave_id;
        cfg.tcp_port = port;
        let mut client = tcp::Transport::new_with_cfg(ip, cfg)?;
        thread::sleep(time::Duration::from_millis(1000));
        Ok(HuaweiSolar { client: client })
    }

    pub fn read_numeric_register<T: NumericRegisterTrait>(&mut self, reg: &T) -> Result<f64, modbus::Error> {
        match reg.read(&mut self.client) {
            Ok(v) => Ok(v[0]),
            Err(e) => Err(e)
        }
    }

    pub fn read_string_register(&mut self, reg: &StringRegister) -> Result<String, modbus::Error> {
        reg.read(&mut self.client)
    }

    pub fn read_device_status(&mut self, reg: &NumericRegister<u16>) -> Result<String, modbus::Error> {
        let status = self.read_numeric_register(reg)?;
        let mut result = String::new();
        for (code, desc) in DEVICE_STATUS_DEFINITIONS.iter() {
            if status as u16 == *code {
                result = desc.to_string();
            }
        }
        Ok(result)
    }
}





