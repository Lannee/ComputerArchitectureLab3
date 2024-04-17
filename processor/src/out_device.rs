use crate::{cpu::ports::Port, input::IntSchedule};


#[derive(Debug)]
pub struct Device<'a> {
    schedule: IntSchedule,
    pub port: Option<&'a mut Port<'a>>,

    _tick: usize,
}

impl<'a> Device<'a> {
    pub fn new(schedule: IntSchedule, port: Option<&'a mut Port<'a>>) -> Device<'a> {
        Device {
            schedule,
            port,

            _tick: 0,
        }
    }

    pub fn tick(&mut self) {
        if let Some(data) = self.schedule.iter().filter(|interupt| interupt.0 == self._tick).next() {
            self._out(data.1 as u8);
        }
        self._tick += 1;
    }

    pub fn _in(&mut self, data: u8) {
        print!("{}", data as char);
    }

    pub fn _out(&mut self, data: u8) {
        if let Some(port) = &mut self.port {
            port._in(data);
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