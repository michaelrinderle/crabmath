use crate::types::Fraction;

use std::ops::Mul;

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Fraction::new(numerator, denominator).unwrap()
    }
}

#[cfg(test)]
mod fraction_mul_tests {
    use super::*;

    #[test]
    fn fraction_mul_test() {
        let frac1 = Fraction::new(1, 2).unwrap();
        let frac2 = Fraction::new(1, 2).unwrap();
        let frac3 = frac1 * frac2;

        assert_eq!(frac3.numerator, 1);
        assert_eq!(frac3.denominator, 4);
    }
}
