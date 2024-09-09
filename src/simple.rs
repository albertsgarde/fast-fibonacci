use crate::FibonnaciCalculator;

pub struct Simple;

impl FibonnaciCalculator for Simple {
    fn calculate(&self, index: u64) -> num::BigUint {
        if index == 0 {
            num::BigUint::from(0u8)
        } else if index == 1 {
            num::BigUint::from(1u8)
        } else {
            let mut a = num::BigUint::from(0u8);
            let mut b = num::BigUint::from(1u8);
            for _ in 1..index {
                a += &b;
                std::mem::swap(&mut a, &mut b);
            }
            b
        }
    }
}

#[cfg(test)]
mod test {
    use crate::naive::Naive;

    use super::*;

    #[test]
    fn test_calculate() {
        for index in 0..25 {
            assert_eq!(Simple.calculate(index), Naive.calculate(index));
        }
    }
}
