use std::collections::VecDeque;

pub(crate) struct RingBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn get_last(&self) -> Option<&T> {
        self.buffer.back()
    }

    pub fn increase_capacity(&mut self, new_capacity: usize) {
        if new_capacity > self.capacity {
            // Create a new buffer with the new capacity
            let mut new_buffer = VecDeque::with_capacity(new_capacity);
            
            // Move elements from the old buffer to the new buffer
            while let Some(item) = self.buffer.pop_front() {
                new_buffer.push_back(item);
            }
            
            self.buffer = new_buffer;
            self.capacity = new_capacity;
        }
    }

    pub fn iter(&self) -> RingBufferIterator<T> {
        RingBufferIterator {
            buffer: &self.buffer,
            index: 0,
        }
    }
}

pub struct RingBufferIterator<'a, T> {
    buffer: &'a VecDeque<T>,
    index: usize,
}

impl<'a, T> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buffer.len() {
            let item = &self.buffer[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T> Default for RingBuffer<T> {
    fn default() -> Self {
        Self::new(10)
    }
}
