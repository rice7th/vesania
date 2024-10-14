use glam::{Vec2, Vec4};
use crate::shape::Shape;

use super::{lerp, Bezier};

#[derive(Debug)]
pub struct Line {
    a: Vec2,
    b: Vec2
}

impl Line {
    pub fn new(a: Vec2, b: Vec2) -> Line {
        return Line { a, b };
    }
}

impl Bezier for Line {
    fn t(&self, t: f32) -> Vec2 {
        return Vec2::new(
            lerp(self.a.x, self.b.x, t),
            lerp(self.a.y, self.b.y, t)
        );
    }
    
    fn bb(&self) -> glam::Vec4 {
        return Vec4::new(
            f32::min(self.a.x, self.b.x), f32::min(self.a.y, self.b.y),
            f32::max(self.a.x, self.b.x), f32::max(self.a.y, self.b.y),
        )
    }

    fn slope(&self, t: f32) -> f32 {
        return (self.b.y - self.a.y) / (self.b.x - self.a.x);
    }
}

impl Shape for Line {
    fn intersections(&self, p: Vec2) -> Vec<f32> {
        if p.y > f32::max(self.a.y, self.b.y) { return vec![]; }
        if self.a.x == self.b.x {
            return vec![self.a.x];
        } else {
            let m = (self.b.y - self.a.y) / (self.b.x - self.a.x);
            let i = (p.y - self.a.y) / m + self.a.x;
            return vec![i];
        }
    }
}