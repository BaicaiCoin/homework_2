use std::ops::Add;
use std::vec::Vec;

pub struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Buffer{data: Vec::new(),}
    }
    pub fn sum(&self) -> T
    where T: Add<Output = T> + Default + Copy
    {
        self.data.iter().fold(T::default(), |acc, &x| acc+x)
    }
    pub fn push(&mut self, element: T) {
        self.data.push(element);
    }
}