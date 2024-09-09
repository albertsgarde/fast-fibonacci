use crate::FibonnaciCalculator;

pub struct Naive;

impl FibonnaciCalculator for Naive {
    fn calculate(&self, index: u64) -> num::BigUint {
        if index == 0 {
            num::BigUint::from(0u8)
        } else if index == 1 {
            num::BigUint::from(1u8)
        } else {
            Naive.calculate(index - 1) + Naive.calculate(index - 2)
        }
    }
}
