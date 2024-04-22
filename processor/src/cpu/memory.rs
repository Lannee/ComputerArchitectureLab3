use crate::input::machine_code::Address;




#[derive(Debug)]
pub struct Memory<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T: Default> Memory<T> {
    pub fn from_data_with_spec_size(mut data: Vec<T>, capacity: usize) -> Memory<T> {
        Memory {
            data: {data.resize_with(capacity, Default::default); data},
            capacity
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Memory<T> {
    pub fn read(&self, index: Address) -> &T {
        self.data.get(index as usize % self.capacity).expect("Invalid address read!")
    }

    pub fn write(&mut self, index: Address, value: T) {
        self.data[index as usize % self.capacity] = value;
    }
}


pub type ByteMemory = Memory<u8>;

impl ByteMemory {
    pub fn read_w(&self, index: Address) -> u32 {
        let bytes = [
            *self.read(index),
            *self.read(index + 1),
            *self.read(index + 2),
            *self.read(index + 3),
        ];
        u32::from_le_bytes(bytes)
    }

    pub fn write_w(&mut self, index: Address, value: u32) {
        value.to_le_bytes().iter().enumerate().for_each(|(i, x)| self.write(index.overflowing_add(i as u32).0, *x));
    }
}
