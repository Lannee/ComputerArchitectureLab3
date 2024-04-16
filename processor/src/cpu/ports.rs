use std::{cell::RefCell, ops::Deref, rc::Rc};

use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use crate::out_device::Device;




pub struct Port {
    pub data: u8,
    device: Option<Rc<RefCell<Device>>>
}

impl Port {
    pub fn new(device: Option<Rc<RefCell<Device>>>) -> Port {
        Port {
            data: u8::default(),
            device,
        }
    }

    pub fn out(&mut self, data: u8) {
        self.data = data;
    }

    pub fn tick(&mut self) {
        if let Some(device) = &mut self.device {
            device.deref().borrow_mut().tick()
        }
    }
}



pub struct IOInterface {
    port1: Rc<RefCell<Port>>,
    port2: Rc<RefCell<Port>>
}

impl IOInterface {
    pub fn new() -> IOInterface {
        IOInterface {
            port1: Rc::new(RefCell::new(Port::new(None))),
            port2: Rc::new(RefCell::new(Port::new(None)))
        }
    }

    pub fn connect_device(&mut self, port: PortSelect, dev: Rc<RefCell<Device>>) {
        match port {
            PortSelect::Port0 => {
                dev.deref().borrow_mut().port = Some(Rc::clone(&self.port1));
                self.port1.deref().borrow_mut().device = Some(dev);
            },
            PortSelect::Port1 => {
                dev.deref().borrow_mut().port = Some(Rc::clone(&self.port2));
                self.port2.deref().borrow_mut().device = Some(dev);
            }
        }
    }

    pub fn tick(&mut self) {
        self.port1.deref().borrow_mut().tick();
        self.port2.deref().borrow_mut().tick();
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