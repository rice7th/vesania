use glam::{Mat2, Vec2, Vec4};
use crate::shape::Shape;

use super::{lerp, line::Line, Bezier};

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

    pub fn is_line(&self) -> bool {
        if (self.b.y - self.a.y) * (self.c.x - self.b.x)
        == (self.c.y - self.b.y) * (self.b.x - self.a.x) {
            return true;
        } else {
            return false;
        }
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
        let max = Vec2::max(self.a, Vec2::max(self.b, self.c)) + 1.;
        let min = Vec2::min(self.a, Vec2::min(self.b, self.c)) - 1.;
        return Vec4::from([min.x, min.y, max.x, max.y]);
    }

    fn first_point(&self) -> &Vec2 {
        &self.a
    }

    fn last_point(&self) -> &Vec2 {
        &self.c
    }

    fn derivative(&self, t: f32) -> Vec2 {
        return self.a * (2.0*t - 2.0) + (2.0*self.c - 4.0*self.b)*t + 2.0*self.b;
    }

    fn second_derivative(&self, _: f32) -> Vec2 {
        return 2.0*(self.c - 2.0*self.b + self.a);
    }

    fn parallel(&self, dist: f32) -> Vec<Box<dyn Bezier>> {
        todo!()
    }

    fn split(&self, t: f32) -> (Box<dyn Bezier>, Box<dyn Bezier>) {
        let d = Vec2::new(lerp(self.a.x, self.b.x, t), lerp(self.a.y, self.b.y, t));
        let e = Vec2::new(lerp(self.b.x, self.c.x, t), lerp(self.b.y, self.c.y, t));
        let f = Vec2::new(lerp(d.x, e.x, t), lerp(d.y, e.y, t));

        return (
            Box::new(Quadratic::new(self.a, d, f)),
            Box::new(Quadratic::new(f, e, self.b)),
        )
    }

    fn fix(&self) -> Vec<Box<dyn Bezier>> {
        // if the control point is either above or below the two other points, the
        // curve will certainly contain a point with a slope of zero, so we split
        // the curve in two at that point.
        if self.b.y > self.bb().w || self.b.y < self.bb().y {
            let t = (2.0*self.a - 2.0*self.b)/(2.0*self.a - 4.0*self.b + 2.0*self.c);
            // t is a Vec2. Technically each component should be equal, but y'know,
            // floating point math sucks.
            let splitted = self.split(t.x);
            return vec![splitted.0, splitted.1];
        } else if self.is_line() {
            // NOTE: a curve should never be colinear! If it is, maybe the control
            // point just lies between the other two points.
            // However, if it is not the case (e.g. the control point is the last one),
            // then it is an edge case that I am not going to address because
            // I don't really care; this type of edge case doesn't normally occour.
            // So I just return a fixed line (imagine if all the points are aligned
            // to the ray) from a to c, which should be the most common solution.
            return Line::new(self.a, self.c).fix();
        }
        return vec![Box::new(Quadratic::new(self.a, self.b, self.c))];
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

        if delta <= -0.0001 { return vec![]; } // No intersections; Because of precision, delta can be negative.
        let delta = f32::max(0.0, delta); // clam delta anyways

        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);

        let t1 = if t1 == 1.0 { 1.0 - 0.001 } else { t1 };
        let t2 = if t2 == 1.0 { 1.0 - 0.001 } else { t2 };

        if t1 <= 1.0 && t1 >= 0.0 { inters.push(t1) }
        if t2 <= 1.0 && t2 >= 0.0 { inters.push(t2) }
        
        return inters;
    }
}