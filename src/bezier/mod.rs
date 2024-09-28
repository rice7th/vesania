use glam::{Vec2, Vec4};

pub mod quadratic;
pub mod line;


pub trait Bezier {
    fn t(&self, t: f32) -> Vec2;
    fn bb(&self) -> Vec4;
}

// bug?
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return (1. - t) * a + t * b;
}