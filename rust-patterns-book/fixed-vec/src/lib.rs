use std::mem::MaybeUninit;

pub struct FixedVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    idx: usize
}

impl<T, const N: usize> FixedVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            idx: 0
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.idx >= N {
            Err(value)
        } else {
            self.data[self.idx] = MaybeUninit::new(value);
            self.idx += 1;
            
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.idx == 0 {
            None 
        } else {
            let result;
            unsafe {
                // SAFETY: we will never read this value again until we store something else
                // there
                result = MaybeUninit::assume_init_read(&self.data[self.idx- 1]); 
            }
            self.idx -= 1;

            Some(result)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        // only return slice of initialized elements
        let result =unsafe { 
            std::slice::from_raw_parts(self.data.as_ptr() as *const T , self.idx)
        };

        result
    }
}

impl<T, const N: usize> Drop for FixedVec<T, N> {
    fn drop(&mut self) {
        // only drop initialized elements
        for i in 0..(self.idx) {
            unsafe {
                // SAFETY: this container is dropped forever; so this memory cannot be accessed
                // through this container
                self.data[i].assume_init_drop();
                /*
                let result = std::ptr::read(&self.data[i]);
                // explicitly drop result here
                drop(result);
                */
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        const N: usize = 10;
        let mut int_vec = FixedVec::<i32, N>::new();
        
        for i in 0..N {
            println!("pushing {:?}", i);
            int_vec.push(i as i32).ok();
        }

        assert_eq!(Err(10), int_vec.push(10));
        let slice =int_vec.as_slice(); 
        let slice2: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(slice, &slice2[..]);

        let mut x = 9;
        while let Some(val) = int_vec.pop() {
            assert_eq!(val, x);
            x -= 1;
        }

        assert_eq!(x, -1);
    }
}
