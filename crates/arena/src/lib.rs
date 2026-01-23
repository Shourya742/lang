use std::{marker::PhantomData, ops::Index};

#[derive(Debug)]
pub struct Idx<T> {
    raw: u32,
    _phantom: PhantomData<fn() -> T>,
}

#[derive(Debug)]
pub struct Arena<T> {
    data: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn alloc(&mut self, t: T) -> Idx<T> {
        let idx = self.next_idx();
        self.data.push(t);

        idx
    }

    fn next_idx(&self) -> Idx<T> {
        Idx {
            raw: self.data.len() as u32,
            _phantom: PhantomData,
        }
    }
}

impl<T> Index<Idx<T>> for Arena<T> {
    type Output = T;
    fn index(&self, index: Idx<T>) -> &Self::Output {
        &self.data[index.raw as usize]
    }
}
