use crate::define_register32;

pub type Register32 = Register<i32>;

define_register32!{REG0}
define_register32!{REG1}
define_register32!{REG2}
define_register32!{REG3}
define_register32!{REG4}
define_register32!{REG5}
define_register32!{REG6}
define_register32!{REG7}

pub struct Register<T: Copy> {
    value: T
}

impl<T: Copy + Default> Register<T> {
    fn new() -> Register<T> {
        Register { value: T::default() }
    }

    fn get_value(&self) -> T {
        self.value
    }

    fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

#[macro_export]
macro_rules! define_register32 {
    ($name:ident) => {
        pub static mut $name: Register32 = Register32 {
            value: 0 as i32
        };
    };
}