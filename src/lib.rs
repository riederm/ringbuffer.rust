macro_rules! circular_increment {
    ($value:expr, $max:expr) => {
        $value = ($value + 1) % $max;
    };
}


pub struct RingBuffer<T: Copy> {
    data: Vec<Option<T>>,
    write_head : usize,
    read_head : usize
}

impl <T : Copy> RingBuffer<T> {
    pub fn new(ring_size: usize) -> Self {
        RingBuffer {
            data: vec![None; ring_size],    // ring_size x None
            write_head : 0,
            read_head : 0
        }
    }

    pub fn number_of_elements(&self) -> usize {
        self.data.iter().filter(|&item| item.is_some()).count()
    }

    /// adds a new element to the ring,
    /// if the ring is full, the oldest element is woverwritten
    pub fn add(&mut self, element: T) {
        let element_count = self.data.len();
        if self.number_of_elements() == self.data.len(){//buffer full, cycle
            self.data[self.write_head] = Some(element);
            circular_increment!(self.write_head, element_count);
            circular_increment!(self.read_head, element_count);
        }else {
            self.data[self.write_head] = Some(element);
            circular_increment!(self.write_head, element_count);
        }
    }

    /// removes the oldest element from the ring
    pub fn remove(&mut self) -> Option<T> {
        if self.number_of_elements() > 0{
            let ret_val = self.data[self.read_head].take();
            circular_increment!(self.read_head, self.data.len());
            ret_val
        }else {
            None
        }
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

    #[test]
    fn it_should_report_the_correct_number_of_elements_when_filled() {
        //Given an empty buffer
        let mut buffer: RingBuffer<i32> = RingBuffer::<i32>::new(10);
        //when added 4 elements
        for i in 0..4 {
            buffer.add(i);    
        }
        let size = buffer.number_of_elements();
        //then it should return 4
        assert_eq!(size, 4);
    }

    #[test]
    fn it_should_report_the_correct_number_of_elements_when_overfilled() {
        //Given an empty buffer
        let mut buffer: RingBuffer<i32> = RingBuffer::<i32>::new(10);
        //when added 12 elements
        for i in 1..13 {
            buffer.add(i);    
        }
        let size = buffer.number_of_elements();
        //then it should return 10
        assert_eq!(size, 10);
    }

    #[test]
    fn it_should_return_the_before_pushed_values() {
        //Given an empty buffer
        let mut buffer: RingBuffer<i32> = RingBuffer::<i32>::new(10);
        //when added 12 elements
        for i in 1..11 {
            println!("added {i}");
            buffer.add(i);    
        }
        //then it should return 10
        for i in 1..11 {
            let ret_val = buffer.remove();
            println!("expected {}, was {:?}", i , ret_val);
            assert_eq!(ret_val, Some(i)); //Why has this to be some of...? 
        }
    }

    #[test]
    fn it_should_return_none_when_empty() {
        //Given an empty buffer
        let mut buffer: RingBuffer<i32> = RingBuffer::<i32>::new(10);
        //when removing an element
        let ret_val = buffer.remove();
        //then it should return none
        assert!(ret_val.is_none()); 
    }

    #[test]
    fn it_should_write_in_a_circle_when_old_values_are_read() {
        //Given an fully filled  buffer
        let mut buffer: RingBuffer<i32> = RingBuffer::<i32>::new(10);
        for i in 1..11 {
            buffer.add(i);    
        }
        //when adding another 5 elements
        for i in 11..16 {
            buffer.add(i);    
        }
        //then the oldest value should always be returned
        for i in 6..16 {
            let ret_val = buffer.remove();
            println!("expected{}, was {:?}", i, ret_val);
            assert_eq!(ret_val, Some(i));    
        }
    }
}


