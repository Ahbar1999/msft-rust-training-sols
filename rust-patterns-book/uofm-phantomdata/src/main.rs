use std::marker::PhantomData;
use std::ops::{Add, Mul, Div};

// Unit marker types (zero-sized)

#[derive(Debug, Clone, Copy)]
struct Meters;
#[derive(Debug, Clone, Copy)]
struct Seconds;
#[derive(Debug, Clone, Copy)]
struct MetersPerSecond;
#[derive(Debug, Clone, Copy)]
struct Kilograms;
#[derive(Debug, Clone, Copy)]
struct SquareMeters;

#[derive(Debug, Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Quantity<U> {
    fn new(value: f64) -> Self {
        Quantity { value, _unit: PhantomData }
    }
}

// Can only add same units:
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

// Meters / Seconds = MetersPerSecond (custom trait)
impl Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Quantity<MetersPerSecond> {
        Quantity::new(self.value / rhs.value)
    }
}

// implement Multiplication for same meters 
impl Mul for Quantity<Meters> {
    type Output = Quantity<SquareMeters>;
    
    fn mul(self, rhs: Quantity<Meters>) -> Self::Output {
        Quantity::<SquareMeters> { value: self.value * rhs.value, _unit: PhantomData }
    }
}
   
// ability to multiply Quantity<Meters> with constants
impl Mul<f64> for Quantity<Meters> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Quantity::<Meters> { value: self.value * rhs, _unit: PhantomData }  
    } 
}

fn main() { }

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn basics() {
        // let nonsense = dist + time; // ❌ Compile error: can't add Meters + Seconds
        let dist = Quantity::<Meters>::new(100.0);
        let time = Quantity::<Seconds>::new(9.58);
        let speed = dist / time; // Quantity<MetersPerSecond>
        println!("Speed: {:.2} m/s", speed.value); // 10.44 m/s
    }

    pub fn test_mul() {
        let height = Quantity::<Meters>::new(100.0);
        let width = Quantity::<Meters>::new(200.0);
        
        let area: Quantity::<SquareMeters> = height * width;
        let parameter: Quantity::<Meters> = (height + width) * 2.0; 
        
        assert_eq!(area.value, 20000.0);

        assert_eq!(parameter.value, 20000.0);
    }
}

