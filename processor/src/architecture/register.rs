pub static REG0: Register32 = Register32::new(); 
pub static REG1: Register32 = Register32::new(); 
pub static REG2: Register32 = Register32::new(); 
pub static REG3: Register32 = Register32::new(); 
pub static REG4: Register32 = Register32::new(); 
pub static REG5: Register32 = Register32::new(); 
pub static REG6: Register32 = Register32::new(); 
pub static REG7: Register32 = Register32::new(); 

pub type Register32 = Register<i32>;

struct Register<T: Copy> {
    value: T
}

impl<T: Copy + Default> Register<T> {
    fn new() -> Register<T> {
        Register { value: T::default() }
    }
}