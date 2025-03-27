use glam::Vec2;

use crate::bezier::line::Line;

/// # GridSegments
/// A simple wrapper around a more complex type.
/// It's basically a tensor of Lines.
/// The x and y dimensions represent the segment's
/// x and y position in the canvas, while the z
/// dimension represents the Zth segment in said pixel
pub type GridSegments = Vec<Vec<Vec<Line>>>;

/// # Shape
/// A common interface for rendering shapes.
pub trait Shape {
    /// # Grid Intersections
    /// Returns a list of segments split
    /// at the intersections with the pixel grid
    fn grid_intersections(&self, grid: &mut GridSegments);
}
