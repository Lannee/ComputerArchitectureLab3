use std::{cell::RefCell, ops::Deref, rc::Rc};

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
            PortSelect::Port1 => {
                dev.deref().borrow_mut().port = Some(Rc::clone(&self.port1));
                self.port1.deref().borrow_mut().device = Some(dev);
            },
            PortSelect::Port2 => {
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
    Port1,
    Port2,
}