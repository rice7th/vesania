use glam::{Vec2, Vec4};
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

        return Vec2::new(lerp(d.x, e.x, t), lerp(d.y, e.y, t));
    }

    fn bb(&self) -> Vec4 {
        // FIXME: This approach sucks. Actual quadratic beziers AABBs
        // use simple derivatives to figure out local minima and maxima
        // of the function. The funny thing is that I have no idea how
        // to take derivatives :'(
        // For now let's just take the AABB of the control points.
        // Addendum 14/OCT/2024: I found out the derivative of a bezier
        // (Yay!) And I can calculate slopes for both curves and lines.
        // Unfortunately I still have NO IDEA how to derive these myself,
        // let alone figure out minima and maxima.
        let max = Vec2::max(self.a, Vec2::max(self.b, self.c));
        let min = Vec2::min(self.a, Vec2::min(self.b, self.c));
        return Vec4::from([min.x, min.y, max.x, max.y]);
    }

    fn slope(&self, t: f32) -> f32 {
        let q = self.a * (2.0*t - 2.0) + (2.0*self.c - 4.0*self.b)*t + 2.0*self.b;
        return q.y / q.x;
    }
}

// TODO: Add Epsilon values because floating point math sucks
impl Shape for Quadratic {
    fn intersections(&self, p: Vec2) -> Vec<f32> {
        // Since we're shooting horizontal rays, we only need to care
        // about the y components of the bezier. The intersections
        // can be found by solving the parabola equation formed by
        // the y component of the quadratic curve:
        // at² + bt + c = 0
        // 
        // The coefficients are derived from the control points as
        // specified below:
        let mut inters = vec![];
        let a = self.a.y - 2.0*self.b.y + self.c.y;
        let b = 2.0 * (self.b.y - self.a.y);
        let c = self.a.y - p.y;
        let delta = b*b - 4.0*a*c;

        if delta <= 0.0 { return vec![]; } // No intersections

        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);

        if t1 <= 1.0 && t1 >= 0.0 { inters.push(t1) }
        if t2 <= 1.0 && t2 >= 0.0 { inters.push(t2) }

        return inters;
    }
}