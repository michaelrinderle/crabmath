//! This module provides functionality for calculating the area of various geometric shapes.
//!
//! It is part of the `geometry` module and includes functions and structures to handle
//! area calculations for different shapes, such as rectangles, circles, triangles, etc.
//!
//! # Examples
//!
//! ```rust
//! use crabmath::fields::geometry::area;
//!
//! let area_circle = area::get_area_circle(12.0);
//! let area_parallelogram = area::get_area_parallelogram(15.5, 5.5);
//! let area_rectangle = area::get_area_rectangle(3.5, 3.5);
//! let area_square = area::get_area_square(3.5);
//! let area_trapezoid = area::get_area_trapezoid(15.5, 15.5, 10.0);
//! let area_triangle = area::get_area_triangle(15.5, 30.5);
//! let area_triangle_right = area::get_area_triangle_right(15.5, 30.5);
//!
//! assert_eq!(area_circle, 452.3893421169302);
//! assert_eq!(area_parallelogram, 85.25);
//! assert_eq!(area_rectangle, 12.25);
//! assert_eq!(area_square, 12.25);
//! assert_eq!(area_trapezoid, 155.0);
//! assert_eq!(area_triangle, 236.375);
//! assert_eq!(area_triangle_right, 236.375);
//! ```
//!
//! # Functions
//!
//! - `get_area_circle`: Computes the area of a circle.
//! - `get_area_parallelogram`: Computes the area of a parallelogram.
//! - `get_area_rectangle`: Computes the area of a rectangle.
//! - `get_area_square`: Computes the area of a square.
//! - `get_area_trapezoid`: Computes the area of a trapezoid.
//! - `get_area_triangle` : Computes the area of a triangle.
//! - `get_area_triangle_right` : Computes the area of a right triangle.

use num_traits::{Num, NumCast};

// Function to get area of a circle
pub fn get_area_circle<T: Num>(radius: T) -> T
where
    T: Num + NumCast,
{
    let radius_f64 = T::to_f64(&radius).unwrap();
    let area_f64 = std::f64::consts::PI * radius_f64.powi(2);
    T::from(area_f64).unwrap()
}

// Function to get area of a parallelogram
pub fn get_area_parallelogram<T: Num>(base: T, height: T) -> T {
    base * height
}

// Function to get area of a rectangle
pub fn get_area_rectangle<T: Num>(length: T, width: T) -> T {
    length * width
}

// Function to get area of a square.
pub fn get_area_square<T: Num>(side: T) -> T
where
    T: Num + NumCast,
{
    let side_f64 = T::to_f64(&side).unwrap();
    T::from(side_f64.powi(2)).unwrap()
}

// Function to get area of a trapezoid.
pub fn get_area_trapezoid<T: Num>(base1: T, base2: T, height: T) -> T
where
    T: Num + NumCast,
{
    let base1_f64 = T::to_f64(&base1).unwrap();
    let base2_f64 = T::to_f64(&base2).unwrap();
    let height_f64 = T::to_f64(&height).unwrap();

    T::from(0.5f64 * (base1_f64 + base2_f64) * height_f64).unwrap()
}

// Function to get area of a triangle.
pub fn get_area_triangle<T: Num>(base: T, height: T) -> T
where
    T: Num + NumCast,
{
    let base_f64 = T::to_f64(&base).unwrap();
    let height_f64 = T::to_f64(&height).unwrap();
    T::from(0.5f64 * base_f64 * height_f64).unwrap()
}

// Function to get area of a right triangle.
pub fn get_area_triangle_right<T: Num>(adjacent: T, opposite: T) -> T
where
    T: Num + NumCast,
{
    let adjacent_f64 = T::to_f64(&adjacent).unwrap();
    let opposite_f64 = T::to_f64(&opposite).unwrap();
    T::from(0.5f64 * adjacent_f64 * opposite_f64).unwrap()
}

#[cfg(test)]
mod geometry_area_tests {
    use super::*;

    #[test]
    fn get_area_circle_test() {
        let result = get_area_circle(12.0);
        assert_eq!(result, 452.3893421169302);

        let result = get_area_circle(12);
        assert_eq!(result, 452);
    }

    #[test]
    fn get_area_parallelogram_test() {
        let result = get_area_parallelogram(15.5, 5.5);
        assert_eq!(result, 85.25);

        let result = get_area_parallelogram(15, 5);
        assert_eq!(result, 75);
    }

    #[test]
    fn get_area_rectangle_test() {
        let result = get_area_rectangle(3.5, 3.5);
        assert_eq!(result, 12.25);

        let result = get_area_rectangle(3, 3);
        assert_eq!(result, 9);
    }

    #[test]
    fn get_area_square_test() {
        let result = get_area_square(3.5);
        assert_eq!(result, 12.25);

        let result = get_area_square(3);
        assert_eq!(result, 9);
    }

    #[test]
    fn get_area_trapezoid_test() {
        let result = get_area_trapezoid(15.5, 15.5, 10.0);
        assert_eq!(result, 155.0);

        let result = get_area_trapezoid(15, 15, 10);
        assert_eq!(result, 150);
    }

    #[test]
    fn get_area_triangle_test() {
        let result = get_area_triangle(15.5, 30.5);
        assert_eq!(result, 236.375);

        let result = get_area_triangle(15, 30);
        assert_eq!(result, 225);
    }

    #[test]
    fn get_area_triangle_right_test() {
        let result = get_area_triangle_right(15.5, 30.5);
        assert_eq!(result, 236.375);

        let result = get_area_triangle_right(15, 30);
        assert_eq!(result, 225);
    }
}
