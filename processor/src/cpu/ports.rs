use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use crate::out_device::Device;

#[derive(Debug)]
pub struct Port {
    pub data: u8,
    device: Option<Device>,

    pub int_req: bool,
}

impl Port {
    pub fn new(device: Option<Device>) -> Port {
        Port {
            data: u8::default(),
            device,
            int_req: false,
        }
    }

    pub fn connect_device(&mut self, dev: Device) {
        self.device = Some(dev)
    }

    pub fn _out(&mut self, data: u8) {
        self.data = data;
        if let Some(dev) = &mut self.device {
            dev._in(data);
        }
    }

    pub fn _in(&mut self, data: u8) {
        self.data = data;
    }

    pub fn tick(&mut self) {
        if let Some(device) = &mut self.device {
            device.tick();
            self.int_req = device.int_req;
            self.data = device.on_bus();
        }
    }

    pub fn data_rx(&mut self) {
        if let Some(dev) = &mut self.device {
            dev.data_rx()
        }
    }
}


#[derive(Debug)]
pub struct IOInterface {
    pub port0: Port,
    pub port1: Port,

    pub selected: PortSelect,
    pub data: u8,

    pub int_req: bool,
    pub int_port: PortSelect,
}

impl IOInterface {
    pub fn new() -> IOInterface {
        IOInterface {
            port0: Port::new(None),
            port1: Port::new(None),

            selected: PortSelect::Port0,
            data: 0,
            
            int_req: false,
            int_port: PortSelect::Port0,
        }
    }

    pub fn tick(&mut self) {
        self.port0.tick();
        self.port1.tick();
        
        match self.selected {
            PortSelect::Port0 => {
                self.int_req = self.port0.int_req;
                self.data = self.port0.data;
                if self.port0.int_req {self.int_port = PortSelect::Port0}
            },
            PortSelect::Port1 => {
                self.int_req = self.port1.int_req;
                self.data = self.port1.data;
                if self.port1.int_req {self.int_port = PortSelect::Port1}
            },
        }
    }

    pub fn select_port(&mut self, port: PortSelect) {
        self.selected = port;
    }

    pub fn output(&mut self) {
        match self.selected {
            PortSelect::Port0 => self.port0._out(self.data),
            PortSelect::Port1 => self.port1._out(self.data),
        }
    }

    pub fn data_rx(&mut self) {
        match self.selected {
            PortSelect::Port0 => self.port0.data_rx(),
            PortSelect::Port1 => self.port1.data_rx(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PortSelect {
    Port0 = 0,
    Port1 = 1,
}

impl core::fmt::Display for PortSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_port_id())
    }
}

impl PortSelect {
    pub fn get_port_id(&self) -> usize {
        match self {
            PortSelect::Port0 => 0,
            PortSelect::Port1 => 1,
        }
    }
}


impl<'a> Deserialize<'a> for PortSelect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'a> {
        deserializer.deserialize_u64(__PortSelectVisitor)
    }
}

struct __PortSelectVisitor;

impl<'de> Visitor<'de> for __PortSelectVisitor {
    type Value = PortSelect;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("PortSelect")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(PortSelect::Port0),
            1 => Ok(PortSelect::Port1),
            /* ... */
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(value),
                &"port index",
            )),
        }
    }
}