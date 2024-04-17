use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use crate::out_device::Device;

pub struct Port<'a> {
    pub data: u8,
    device: Option<Device<'a>>
}

impl<'a> Port<'a> {
    pub fn new(device: Option<Device>) -> Port {
        Port {
            data: u8::default(),
            device,
        }
    }

    pub fn _out(&mut self, data: u8) {
        self.data = data;
        if let Some(dev) = &mut self.device {
            dev._in(data);
        }
    }

    pub fn _in(&mut self, data: u8) {

    }

    pub fn tick(&mut self) {
        if let Some(device) = &mut self.device {
            device.tick()
        }
    }
}



pub struct IOInterface<'a> {
    port1: Port<'a>,
    port2: Port<'a>
}

impl<'a> IOInterface<'a> {
    pub fn new() -> IOInterface<'a> {
        IOInterface {
            port1: Port::new(None),
            port2: Port::new(None)
        }
    }

    #[you_can::turn_off_the_borrow_checker]
    pub fn connect_device(&mut self, port: PortSelect, mut dev: Device<'a>) {
        match port {
            PortSelect::Port0 => {
                dev.port = Some(&mut self.port1);
                self.port1.device = Some(dev);
            },
            PortSelect::Port1 => {
                dev.port = Some(&mut self.port2);
                self.port2.device = Some(dev);
            }
        }
    }

    pub fn tick(&mut self) {
        self.port1.tick();
        self.port2.tick();
    }
}

pub enum PortSelect {
    Port0,
    Port1,
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