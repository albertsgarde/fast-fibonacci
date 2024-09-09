use num::BigUint;

pub struct FibonnaciMatrix {
    a: BigUint,
    b: BigUint,
    c: BigUint,
}

impl FibonnaciMatrix {
    pub fn zero() -> Self {
        FibonnaciMatrix {
            a: BigUint::from(1u8),
            b: BigUint::from(0u8),
            c: BigUint::from(1u8),
        }
    }

    pub fn one() -> Self {
        FibonnaciMatrix {
            a: BigUint::from(0u8),
            b: BigUint::from(1u8),
            c: BigUint::from(1u8),
        }
    }

    pub fn b(&self) -> &BigUint {
        &self.b
    }

    pub fn multiply(&self, other: &Self) -> Self {
        FibonnaciMatrix {
            a: &self.a * &other.a + &self.b * &other.b,
            b: &self.a * &other.b + &self.b * &other.c,
            c: &self.b * &other.b + &self.c * &other.c,
        }
    }

    pub fn square(&self) -> Self {
        let b_squared = &self.b * &self.b;
        FibonnaciMatrix {
            a: &self.a * &self.a + &b_squared,
            b: &self.b * (&self.a + &self.c),
            c: &b_squared + &self.c * &self.c,
        }
    }
}
