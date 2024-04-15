use crate::input::machine_code::Instructions;





pub struct Memory<T>(pub Vec<T>);

impl<T: Default> Memory<T> {
    pub fn with_capacity(capacity: usize) -> Memory<T> {
        Memory(Vec::<T>::with_capacity(capacity))
    }

    pub fn from_data_with_spec_size(mut data: Vec<T>, capacity: usize) -> Memory<T> {
        Memory({data.resize_with(capacity, Default::default); data})
    }

    pub fn read(&self, index: usize) -> &T {
        self.0.get(index).expect("Invalid address read!")
    }

    pub fn write(&mut self, index: usize, value: T) {
        self.0[index] = value;
    }
}
