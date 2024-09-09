use crate::{matrix::FibonnaciMatrix, FibonnaciCalculator};

pub struct Power;

impl FibonnaciCalculator for Power {
    fn calculate(&self, mut index: u64) -> num::BigUint {
        let mut matrix = FibonnaciMatrix::one();
        let mut result_matrix = FibonnaciMatrix::zero();
        while index > 0 {
            if index & 1 == 1 {
                result_matrix = result_matrix.multiply(&matrix);
            }
            index >>= 1;
            if index > 0 {
                matrix = matrix.square();
            }
        }
        result_matrix.b().clone()
    }
}

#[cfg(test)]
mod test {
    use crate::simple::Simple;

    use super::*;

    #[test]
    fn test_calculate() {
        for index in 0..2000 {
            assert_eq!(
                Power.calculate(index),
                Simple.calculate(index),
                "index: {}",
                index
            );
        }
    }
}
