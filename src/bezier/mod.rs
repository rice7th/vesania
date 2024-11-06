use std::fmt::Debug;

use glam::{Mat2, Vec2, Vec4};

use crate::shape::Shape;

pub mod quadratic;
pub mod line;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Direction {
    Up, Down
}

/// # Bezier
/// This trait provides common interfaces for various
/// types of bezier curves. By default only Line and
/// Quadratic are implemented, leaving higer order
/// curves up to the user.
pub trait Bezier: Shape + Debug {
    /// # T value
    /// Get the coordinates of the point at `t`.
    /// Notice: while `t` should be in between 0
    /// and 1, nothing stops you from inputting any
    /// other number.
    fn t(&self, t: f32) -> Vec2;
    
    /// # Bounding Box
    /// Get the AABB of the curve.
    fn bb(&self) -> Vec4;

    /// # Derivative
    /// Get the first derivative of a curve at `t`.
    fn derivative(&self, t: f32) -> Vec2;

    /// # Second Derivative
    /// Get the second derivative of a curve at `t`.
    fn second_derivative(&self, t: f32) -> Vec2;

    /// # Slope
    /// Get the slope of the curve at `t`.
    fn slope(&self, t: f32) -> f32 {
        let d = self.derivative(t);
        return d.y / d.x;
    }

    /// # Normal
    /// Get the normal vector at `t`.
    fn normal(&self, t: f32) -> Vec2 {
        let d = self.derivative(t);
        let q = d.length();
        return Vec2::new(d.y / q, -d.x / q);
    }

    /// # Curvature
    /// Get the curvature of the curve at `t`.
    fn curvature(&self, t: f32) -> f32 {
        let first = self.derivative(t);
        let second = self.second_derivative(t);
        let velocity = first.length();
        return Mat2::from_cols(first, second).determinant() / (velocity * velocity * velocity);
    }

    /// # Direction
    /// Get the direction of the curve.
    /// This method differs from slope since
    /// it depends on the position of the starting
    /// and ending points of the curve rather than
    /// its derivative. Thus, the same curve can have
    /// two different directions based on the order
    /// of its points.
    /// 
    // NOTE: After fixing the curve, the curve start
    // and end points are *guaranteed* to have
    // different heights.
    fn direction(&self) -> Direction {
        if self.first_point().y < self.last_point().y {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    // Very hand function for easy direction implementation.

    /// # First
    /// Get first point of a curve.
    /// The first point is always `self.a` for builtin
    /// beziers.
    fn first_point(&self) -> &Vec2;

    /// # Last
    /// Get last point of a curve;
    fn last_point(&self) -> &Vec2;

    /// # Parallel
    /// get the parallel curve of the Bezier.
    /// The Bezier may be split in more than one
    /// piece for better approximations.
    fn parallel(&self, dist: f32) -> Vec<Box<dyn Bezier>>;

    /// # Split
    /// Splits the curve at `t` into two
    /// curves.
    fn split(&self, t: f32) -> (Box<dyn Bezier>, Box<dyn Bezier>);

    /// # Fix
    /// Returns either the same curve, a different
    /// one, nothing or two separate ones, depending
    /// on which edge case the curve represents.
    /// Examples include lines that are perfectly
    /// aligned with a ray (deleted), Quadratic
    /// beziers that include at least one point with
    /// a slope of zero (split into two), or a valid
    /// curve (left as-is).
    /// The function can also be used to fix broken
    /// curves, such as wrongly winded ones.
    fn fix(&self) -> Vec<Box<dyn Bezier>>;
}

// bug?
/// # Lerp
/// Simple linear interpolation between two `f32`s.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return (1. - t) * a + t * b;
}