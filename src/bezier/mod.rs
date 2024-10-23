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

    /// # Slope
    /// Get the slope of the curve at `t`.
    fn slope(&self, t: f32) -> f32;

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