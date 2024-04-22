pub type Register32 = Register<u32>;

#[derive(Debug)]
pub struct Register<T: Copy> {
    pub value: T
}

impl<T: Copy + Default> Register<T> {
    pub fn new() -> Register<T> {
        Register { value: T::default() }
    }
}

#[macro_export]
macro_rules! new_register32 {
    () => {
        Register32 {
            value: 0 as u32
        }
    };
}