
pub struct RingBuffer<T: Copy> {
    data: Vec<Option<T>>,
}

impl <T : Copy> RingBuffer<T> {
    pub fn new(ring_size: usize) -> Self {
        RingBuffer {
            data: vec![None; ring_size],    // ring_size x None
        }
    }

    pub fn number_of_elements(&self) -> usize {
        0
    }

    /// adds a new element to the ring,
    /// if the ring is full, the oldest element is woverwritten
    pub fn add(&mut self, element: T) {
    }

    /// removes the oldest element from the ring
    pub fn remove(&mut self) -> Option<T> {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_ring_is_empty() {
        // GIVEN a new, empty ring
        let buffer = RingBuffer::<i32>::new(10);
        // WHEN I ask for its size
        let size = buffer.number_of_elements();
        // THEN it should be zero
        assert_eq!(size, 0);
    }
}
