//! `Fraction` is a struct representing a mathematical fraction
//!
//! A `Fraction` consists of a numerator and a denominator, and
//! provides various methods to perform operations.
//!
//! # Examples
//!
//! ```rust
//! use crabmath::types::Fraction;
//!
//! let mut frac = Fraction::new(2, 4).unwrap();
//!
//! frac.simplify();
//!
//! let reciprocal = frac.reciprocal();
//! let decimal = frac.to_decimal();
//! let string = frac.to_string();
//!
//! let gcd = Fraction::gcd(2, 4);
//! let lcm = Fraction::lcm(2, 4);
//!
//! assert_eq!(frac.numerator, 1);
//! assert_eq!(frac.denominator, 2);
//! assert_eq!(reciprocal.numerator, 2);
//! assert_eq!(reciprocal.denominator, 1);
//! assert_eq!(decimal, 0.5);
//! assert_eq!(string, "1/2");
//! assert_eq!(gcd, 2);
//! assert_eq!(lcm, 4);
//!```
//!
//! # Functions
//!
//! - `new`: Constructs a new fraction structure
//! - `gcd`: Finds the greatest common denominator.
//! - `lcm`: Finds least common multiple.
//! - `reciprocal`: Finds the fractions reciprocal.
//! - `simplify`: Simplifies the fraction structure.
//! - `to_decimal`: Gets the fraction structure's decimal value.
//! - `to_string`: Gets the fraction structure's string value.

use crate::types::fraction::error::FractionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fraction {
    pub numerator: isize,
    pub denominator: usize,
}

impl Fraction {
    // Function to create a new Fraction instance.
    pub fn new(numerator: isize, denominator: usize) -> Result<Self, FractionError> {
        // Check if the denominator is zero, which is not allowed.
        if denominator == 0 {
            return Err(FractionError::new("Denominator cannot be 0"));
        }
        // Return the new Fraction instance.
        Ok(Fraction {
            numerator,
            denominator,
        })
    }

    // Function to calculate the greatest common divisor (GCD) of two numbers.
    pub fn gcd(mut a: usize, mut b: usize) -> usize {
        // Use the Euclidean algorithm to find the GCD.
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        // Return the GCD
        a
    }

    // Function to calculate the least common multiple (LCM) of two numbers.
    pub fn lcm(a: usize, b: usize) -> usize {
        let gcd = Fraction::gcd(a, b);
        // LCM(a, b) = (a * b) / GCD(a, b)
        a * (b / gcd)
    }

    // Function to get reciprocal of a fraction
    pub fn reciprocal(&self) -> Self {
        let numerator = if self.numerator < 0 {
            -(self.denominator as isize)
        } else {
            self.denominator as isize
        };

        let denominator = self.numerator.abs() as usize;

        Fraction {
            numerator,
            denominator,
        }
    }

    // Function to simplify the fraction by dividing numerator and denominator by their GCD.
    pub fn simplify(&mut self) {
        let gcd = Fraction::gcd(self.numerator.abs() as usize, self.denominator);
        // If the numerator is negative, keep it negative after simplification.
        if self.numerator < 0 {
            // Find GCD without negative sign and reassign numerator.
            let numerator_abs = self.numerator.abs() as usize / gcd;
            self.numerator = -(numerator_abs as isize);
        } else {
            // Otherwise, simply divide the numerator by the GCD.
            self.numerator = self.numerator / gcd as isize;
        }
        // Simplify the denominator by dividing the GCD.
        self.denominator = self.denominator / gcd;
    }

    // Function to convert the fraction to a decimal (integer division).
    pub fn to_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    // Function to convert the fraction into a string.
    pub fn to_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod fraction_tests {
    use super::*;

    #[test]
    fn fraction_new_test() {
        let frac = Fraction::new(1, 2).unwrap();

        assert_eq!(frac.numerator, 1);
        assert_eq!(frac.denominator, 2);
    }

    #[test]
    fn fraction_gcd_test() {
        let frac_one = Fraction::new(1, 2).unwrap();
        let frac_two = Fraction::new(2, 4).unwrap();
        let gcd = Fraction::gcd(frac_one.denominator, frac_two.denominator);

        assert_eq!(gcd, 2);
    }

    #[test]
    fn fraction_lcm_test() {
        let a = 2;
        let b = 4;
        let lcm = Fraction::lcm(a, b);

        assert_eq!(lcm, 4);
    }

    #[test]
    fn fraction_reciprocal() {
        let frac = Fraction::new(1, 2).unwrap();
        let reciprocal = frac.reciprocal();

        assert_eq!(reciprocal.numerator, 2);
        assert_eq!(reciprocal.denominator, 1);
    }

    #[test]
    fn fraction_simplify_test() {
        let mut frac = Fraction::new(2, 4).unwrap();
        frac.simplify();

        assert_eq!(frac.numerator, 1);
        assert_eq!(frac.denominator, 2);
    }

    #[test]
    fn fraction_to_decimal_test() {
        let frac = Fraction::new(1, 2).unwrap();
        let decimal = frac.to_decimal();

        assert_eq!(decimal, 0.5f64);
    }

    #[test]
    fn fraction_to_string_test() {
        let frac = Fraction::new(1, 2).unwrap();
        let string = frac.to_string();

        assert_eq!(string, "1/2");
    }
}
