


pub struct Clock(pub usize);

impl Clock {
    pub fn new() -> Self {
        Clock(0)
    }

    pub fn tick(&mut self) {
        self.0 += 1;
    }

    pub fn get_tick(&self) -> usize {
        self.0
    }
}