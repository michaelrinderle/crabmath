//! This module provides functionality for calculating the circumference of various geometric shapes.
//!
//! It is part of the `geometry` module and includes functions and structures to handle
//! circumference calculations for different shapes, such as circles, etc.
//!
//! # Examples
//!
//! ```rust
//! use crabmath::fields::geometry::circumference;
//!
//! let circumference = circumference::get_circumference(15.5);
//!
//! assert_eq!(circumference, 97.389372261283583);
//! ```
//!
//! # Functions
//!
//! - `get_circumference`: Computes the circumference of a circle.

use num_traits::{Num, NumCast};

// Function to get circumference of a circle.
pub fn get_circumference<T: Num>(radius: T) -> T
where
    T: Num + NumCast,
{
    let radius_f64 = T::to_f64(&radius).unwrap();
    T::from(2f64 * std::f64::consts::PI * radius_f64).unwrap()
}

#[cfg(test)]
mod geometry_circumference_tests {
    use super::*;

    #[test]
    fn get_circumference_test() {
        let result = get_circumference(15.5);
        assert_eq!(result, 97.389372261283583);

        let result = get_circumference(15);
        assert_eq!(result, 94);
    }
}
