use std::fmt::Debug;

use glam::{Vec2, Vec4};

use crate::shape::Shape;

pub mod quadratic;
pub mod line;

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
}

// bug?
/// # Lerp
/// Simple linear interpolation between two `f32`s.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return (1. - t) * a + t * b;
}