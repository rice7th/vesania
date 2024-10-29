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

    fn derivative(&self, t: f32) -> Vec2 {
        return Vec2::from([
            self.b.x - self.a.x,
            self.b.y - self.a.y
        ])
    }

    fn second_derivative(&self, _: f32) -> Vec2 {
        return Vec2::splat(0.);
    }

    fn curvature(&self, t: f32) -> f32 {
        return 0.; // a line is always flat. No need to calculate this.
    }

    fn split(&self, t: f32) -> (Box<dyn Bezier>, Box<dyn Bezier>) {
        let z = Vec2::new(lerp(self.a.x, self.b.x, t), lerp(self.a.y, self.b.y, t));
        return (
            Box::new(Line::new(self.a, z)),
            Box::new(Line::new(z, self.b)),
        )
    }

    fn fix(&self) -> Vec<Box<dyn Bezier>> {
        if self.a.y == self.b.y { return vec![]; } // Erase the line
        return vec![Box::new(Line::new(self.a, self.b))];
    }

    fn parallel(&self, dist: f32) -> Vec<Box<dyn Bezier>> {
        todo!()
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