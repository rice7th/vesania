use glam::Vec2;

/// # Shape
/// A common interface for rendering shapes.
pub trait Shape {
    /// # Intersections
    /// Returns the t values of the
    /// intersections detected between
    /// the shape and a ray at height `p`.
    fn intersections(&self, p: Vec2) -> Vec<f32>;
}