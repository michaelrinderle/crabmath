use crate::types::Fraction;

use std::ops::Add;

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let common_denominator = Fraction::lcm(self.denominator, rhs.denominator);
        let numerator1 = self.numerator * (common_denominator / self.denominator) as isize;
        let numerator2 = rhs.numerator * (common_denominator / rhs.denominator) as isize;
        let result_numerator = numerator1 + numerator2;
        Fraction::new(result_numerator, common_denominator).unwrap()
    }
}

#[cfg(test)]
mod fraction_add_tests {
    use super::*;

    #[test]
    fn fraction_add_test() {
        let frac1 = Fraction::new(1, 2).unwrap();
        let frac2 = Fraction::new(1, 2).unwrap();
        let frac3 = frac1 + frac2;

        assert_eq!(frac3.numerator, 2);
        assert_eq!(frac3.denominator, 2);
    }
}
