use core::fmt;
use std::fmt::Formatter;
use crate::types::Fraction;

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}