use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{cpu::ports::Port, input::IntSchedule};


pub struct Device {
    schedule: IntSchedule,
    pub port: Option<Rc<RefCell<Port>>>,

    _tick: usize,
}

impl Device {
    pub fn new(schedule: IntSchedule, port: Option<Rc<RefCell<Port>>>) -> Device {
        Device {
            schedule,
            port,

            _tick: 0,
        }
    }

    pub fn tick(&mut self) {
        if let Some(data) = self.schedule.iter().filter(|interupt| interupt.0 == self._tick).next() {
            self.out(data.1 as u8);
        }
        self._tick += 1;
    }

    fn out(&mut self, data: u8) {
        match &self.port {
            Some(port) => port.deref().borrow_mut().out(data),
            None => {}
        }
    }
}


// pub struct Device {
//     schedule: IntSchedule,
//     pub port: Option<Box<RefCell<Port>>>,

//     _tick: usize,
// }

// impl Device {
//     pub fn new(schedule: IntSchedule, port: Option<Box<RefCell<Port>>>) -> Device {
//         Device {
//             schedule,
//             port,

//             _tick: 0,
//         }
//     }

//     pub fn tick(&mut self) {
//         if let Some(data) = self.schedule.iter().filter(|interupt| interupt.0 == self._tick).next() {
//             self.out(data.1 as u8);
//         }
//         self._tick += 1;
//     }

//     fn out(&mut self, data: u8) {
//         match &mut self.port {
//             Some(port) => port.out(data),
//             None => {}
//         }
//     }
// }