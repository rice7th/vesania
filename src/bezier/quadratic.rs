use glam::Vec2;
use crate::shape::Shape;

use super::{lerp, Bezier};

/// # Quadratic Bezier curve
/// A 2nd degree Bezier curve defined by three
/// point, named `a`, `b` and `c`.
#[derive(Debug)]
pub struct Quadratic {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}

impl Quadratic {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Quadratic {
        return Quadratic { a, b, c };
    }
}

impl Bezier for Quadratic {
    // TODO: Use a polynomial
    fn t(&self, t: f32) -> Vec2 {
        let a = self.a;
        let b = self.b;
        let c = self.c;

        let d = Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t));
        let e = Vec2::new(lerp(b.x, c.x, t), lerp(b.y, c.y, t));

        return Vec2::new(lerp(d.x, e.x, t), lerp(e.y, d.y, t));
    }

    fn bb(&self) -> glam::Vec4 {
        todo!()
    }
}

impl Shape for Quadratic {
    fn intersections(&self, p: Vec2) -> Vec<f32> {
        todo!()
    }
}