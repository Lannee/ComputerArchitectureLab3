use crate::input::machine_code::Instructions;





pub struct Memory<T>(pub Vec<T>);

impl<T: Default> Memory<T> {
    pub fn with_capacity(capacity: usize) -> Memory<T> {
        Memory(Vec::<T>::with_capacity(capacity))
    }

    pub fn from_data_with_spec_size(mut data: Vec<T>, capacity: usize) -> Memory<T> {
        Memory({data.resize_with(capacity, Default::default); data})
    }
}

impl<T> Memory<T> {
    pub fn read(&self, index: usize) -> &T {
        self.0.get(index).expect("Invalid address read!")
    }

    pub fn write(&mut self, index: usize, value: T) {
        self.0[index] = value;
    }
}


pub type ByteMemory = Memory<u8>;

impl ByteMemory {
    pub fn read_w(&self, index: usize) -> u32 {
        let bytes = [
            *self.read(index),
            *self.read(index + 1),
            *self.read(index + 2),
            *self.read(index + 3),
        ];
        u32::from_le_bytes(bytes)
    }

    pub fn write_w(&mut self, index: usize, value: u32) {
        value.to_le_bytes().iter().enumerate().for_each(|(i, x)| self.write(index + i, *x));
    }
}
