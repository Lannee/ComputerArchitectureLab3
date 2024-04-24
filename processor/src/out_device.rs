use crate::input::IntSchedule;


#[derive(Debug)]
pub struct Device {
    schedule: IntSchedule,
    data: u8,

    // out_buffer: Vec<u8>,
    pub int_req: bool,

    _tick: usize,
}

impl Device {
    pub fn new(schedule: IntSchedule) -> Device {
        Device {
            schedule,
            data: 0,
            // out_buffer: vec![],
            int_req: false,
            

            _tick: 0,
        }
    }

    pub fn tick(&mut self) {
        if let Some(data) = self.schedule.iter().filter(|interupt| interupt.0 == self._tick).next() {
            self._out(data.1);
        }
        self._tick += 1;

        // self.int_req = !self.out_buffer.is_empty();
    }

    pub fn _in(&mut self, data: u8) {
        print!("{}", data as char);
    }

    pub fn _out(&mut self, data: u8) {
        // self.out_buffer.pussh(data);
        self.data = data;
        self.int_req = true;
    }

    pub fn on_bus(&self) -> u8 {
        // *self.out_buffer.first().unwrap_or(&0)
        self.data
    }

    pub fn data_rx(&mut self) {
        // if !self.out_buffer.is_empty() {self.out_buffer.remove(0);};
        self.int_req = false;
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