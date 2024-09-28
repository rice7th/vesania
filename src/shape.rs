use glam::Vec2;

pub trait Shape {
    /// # Winding number
    /// Returns the winding number of the current point.
    fn winding_number(p: Vec2) -> i8;
}