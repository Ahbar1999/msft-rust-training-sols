use std::vec::Vec;

pub struct Pipeline<T> {
    transformations: Vec<Box<dyn FnMut(T) -> T>>
} 

impl<T> Pipeline<T> {
    pub fn new() -> Self {
        Self {
            transformations: Vec::new(),
        }
    }

    pub fn pipe(mut self, f: Box<dyn FnMut(T) -> T>) -> Self {
        self.transformations.push(f);
        self
    }

    pub fn execute(&mut self, input: T) -> Option<T> {
        let mut result = Some(self.transformations[0](input));
            
        for (i, f) in self.transformations.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            
            result = result.map(|value| f(value));
        } 

        result
    } 
}

fn main() {
    // println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut pipeline: Pipeline<i32> = Pipeline::new()
            .pipe(Box::new(|x| x + 1))
            .pipe(Box::new(|x| x * 2))
            .pipe(Box::new(|x| x * x));
        
        // 1 + 1: 2 -> 2 * 2: 4 -> 4 * 4: 16
        assert_eq!(pipeline.execute(1), Some(16));
   }
}


