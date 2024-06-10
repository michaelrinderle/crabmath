use crate::types::Fraction;

use std::ops::Div;

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator as isize;
        let denominator = self.denominator * rhs.numerator.abs() as usize;
        let mut result = Fraction::new(numerator, denominator).unwrap();
        if rhs.numerator < 0 {
            result.numerator = -result.numerator;
        }
        result
    }
}

#[cfg(test)]
mod fraction_div_tests {
    use super::*;

    #[test]
    fn fraction_div_test() {
        let frac1 = Fraction::new(1, 2).unwrap();
        let frac2 = Fraction::new(1, 2).unwrap();
        let frac3 = frac1 / frac2;

        assert_eq!(frac3.numerator, 2);
        assert_eq!(frac3.denominator, 2);
    }
}
