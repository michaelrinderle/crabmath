//! This module provides functionality for calculating the perimeter of various geometric shapes.
//!
//! It is part of the `geometry` module and includes functions and structures to handle
//! perimeter calculations for different shapes, such as circles, etc.
//!
//! # Examples
//!
//! ```rust
//! use crabmath::fields::geometry::perimeter;
//!
//! let perimeter_parallelogram = perimeter::get_perimeter_parallelogram(10.5, 10.5);
//! let perimeter_rectangle = perimeter::get_perimeter_rectangle(10.5, 10.5);
//! let perimeter_square = perimeter::get_perimeter_square(10.5);
//! let perimeter_trapezoid = perimeter::get_perimeter_trapezoid(15.5, 15.5, 15.5, 15.5);
//! let perimeter_triangle = perimeter::get_perimeter_triangle(5.5, 5.5, 5.5);
//!
//! assert_eq!(perimeter_parallelogram, 42.0);
//! assert_eq!(perimeter_rectangle,  42.0);
//! assert_eq!(perimeter_square, 42.0);
//! assert_eq!(perimeter_trapezoid, 62.0);
//! assert_eq!(perimeter_triangle, 16.5);
//! ```
//!
//! # Functions
//!
//! - `get_perimeter_parallelogram`: Computes the perimeter of a parallelogram.
//! - `get_perimeter_rectangle`: Computes the perimeter of a rectangle.
//! - `get_perimeter_square`: Computes the perimeter of a square.
//! - `get_perimeter_trapezoid`: Computes the perimeter of a trapezoid.
//! - `get_perimeter_triangle`: Computers the perimeter of a triangle.

use num_traits::{Num, NumCast};

// Function to get perimeter of parallelogram.
pub fn get_perimeter_parallelogram<T: Num>(adjacent1: T, adjacent2: T) -> T
where
    T: Num + NumCast,
{
    let adjacent1_f64 = T::to_f64(&adjacent1).unwrap();
    let adjacent2_f64 = T::to_f64(&adjacent2).unwrap();
    T::from((2f64 * adjacent1_f64) + (2f64 * adjacent2_f64)).unwrap()
}

// Function to get perimeter of a rectangle.
pub fn get_perimeter_rectangle<T: Num>(length: T, width: T) -> T
where
    T: Num + NumCast,
{
    let length_f64 = T::to_f64(&length).unwrap();
    let width_f64 = T::to_f64(&width).unwrap();
    T::from((2f64 * length_f64) + (2f64 * width_f64)).unwrap()
}

// Function to get perimeter of a square.
pub fn get_perimeter_square<T: Num>(side: T) -> T
where
    T: Num + NumCast,
{
    let side_f64 = T::to_f64(&side).unwrap();
    T::from(4f64 * side_f64).unwrap()
}

// Function to get perimeter of a trapezoid.
pub fn get_perimeter_trapezoid<T: Num>(base1: T, base2: T, leg1: T, leg2: T) -> T
where
    T: Num + NumCast,
{
    let base1_f64 = T::to_f64(&base1).unwrap();
    let base2_f64 = T::to_f64(&base2).unwrap();
    let leg1_f64 = T::to_f64(&leg1).unwrap();
    let leg2_f64 = T::to_f64(&leg2).unwrap();

    T::from(base1_f64 + base2_f64 + leg1_f64 + leg2_f64).unwrap()
}

// Function to get perimeter of a triangle.
pub fn get_perimeter_triangle<T: Num>(a: T, b: T, c: T) -> T
where
    T: Num + NumCast,
{
    let a_f64 = T::to_f64(&a).unwrap();
    let b_f64 = T::to_f64(&b).unwrap();
    let c_f64 = T::to_f64(&c).unwrap();

    T::from(a_f64 + b_f64 + c_f64).unwrap()
}

#[cfg(test)]
mod geometry_perimeter_tests {
    use super::*;

    #[test]
    fn get_perimeter_parallelogram_test() {
        let result = get_perimeter_parallelogram(10.5, 10.5);
        assert_eq!(result, 42.0);

        let result = get_perimeter_parallelogram(10, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn get_perimeter_rectangle_test() {
        let result = get_perimeter_rectangle(10.5, 10.5);
        assert_eq!(result, 42.0);

        let result = get_perimeter_rectangle(10, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn get_perimeter_square_test() {
        let result = get_perimeter_square(10.5);
        assert_eq!(result, 42.0);

        let result = get_perimeter_square(10);
        assert_eq!(result, 40);
    }

    #[test]
    fn get_perimeter_trapezoid_test() {
        let result = get_perimeter_trapezoid(15.5, 15.5, 15.5, 15.5);
        assert_eq!(result, 62.0);

        let result = get_perimeter_trapezoid(15, 15, 15, 15);
        assert_eq!(result, 60);
    }

    #[test]
    fn get_perimeter_triangle_test() {
        let result = get_perimeter_triangle(5.5, 5.5, 5.5);
        assert_eq!(result, 16.5);

        let result = get_perimeter_triangle(5, 5, 5);
        assert_eq!(result, 15);
    }
}
