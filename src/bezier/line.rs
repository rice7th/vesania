use std::{simd::f32x4, sync::Arc};
use glam::{Vec2, Vec4};
use crate::shape::{GridSegments, Shape};

use super::{lerp, Bezier};

#[derive(Debug, Clone, Copy)]
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

    fn first_point(&self) -> &Vec2 {
        &self.a
    }

    fn last_point(&self) -> &Vec2 {
        &self.b
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

    fn split(&self, t: f32) -> Vec<Arc<dyn Bezier>> {
        let z = Vec2::new(lerp(self.a.x, self.b.x, t), lerp(self.a.y, self.b.y, t));
        return vec![
            Arc::new(Line::new(self.a, z)),
            Arc::new(Line::new(z, self.b)),
        ]
    }

    fn fix(&self) -> Vec<Arc<dyn Bezier>> {
        if self.a.y == self.b.y { return vec![]; } // Erase the line
        return vec![Arc::new(Line::new(self.a, self.b))];
    }

    fn parallel(&self, dist: f32) -> Vec<Arc<dyn Bezier>> {
        return vec![self.trans_ctrl_poly(dist)];
    }

    fn trans_ctrl_poly(&self, dist: f32) -> Arc<dyn Bezier> {
        return Arc::new(Line::new(
            self.a + dist*self.normal(0.),
            self.b + dist*self.normal(1.),
        ));
    }
}

impl Shape for Line {
    fn grid_intersections(&self, grid: &mut GridSegments) {
        let mut points = Vec::new();
        let a = self.a;
        let b = self.b;
        let m = (b.y - a.y) / (b.x - a.x);
        let rev_m = 1f32 / m;

        // Vertical grid intersections
        for x in (a.x as u32 .. b.x as u32).step_by(4) {
            let x = x as f32;
            let xvec = f32x4::from_array([x, x + 1., x + 2., x + 3.]);
            let yvec = (xvec - f32x4::splat(a.x)) * f32x4::splat(m) + f32x4::splat(a.y);

            points.push(Vec2::new(xvec[0], yvec[0]));
            points.push(Vec2::new(xvec[1], yvec[1]));
            points.push(Vec2::new(xvec[2], yvec[2]));
            points.push(Vec2::new(xvec[3], yvec[3]));
        }

        // Horizontal grid intersections
        for y in (a.y as u32 .. b.y as u32).step_by(4) {
            let y = y as f32;
            let yvec = f32x4::from_array([y, y + 1., y + 2., y + 3.]);
            let xvec = (yvec - f32x4::splat(a.y)) * f32x4::splat(rev_m) + f32x4::splat(a.x);

            points.push(Vec2::new(xvec[0], yvec[0]));
            points.push(Vec2::new(xvec[1], yvec[1]));
            points.push(Vec2::new(xvec[2], yvec[2]));
            points.push(Vec2::new(xvec[3], yvec[3]));
        }

        // Sort all points
        // FIXME: Different directions?
        // TODO: How performant is this??
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let points = points.iter().filter(|p| p.x <= self.b.x).copied().collect::<Vec<Vec2>>();

        // Connect all the dots into lines
        // TODO: Remove null lines (start/end are the same point)
        // NOTE: These _should_ be sorted by x-position inside the pixel!
        // Otherwise we connect dots in completely random numbers.
        for dots in points.windows(2) {
            // TODO: Make sure the assertions hold
            let x = ((dots[0].x + dots[1].x) / 2.0) as usize;
            let y = ((dots[0].y + dots[1].y) / 2.0) as usize;
            
            if dots[0] == dots[1] { continue; } // Skip

            if dots[0].x <= self.b.x
            && dots[1].x <= self.b.x {
                grid[x][y].push(Line::new(dots[0], dots[1]));
            } else {
                grid[x][y].push(Line::new(dots[0], self.b));
            }
        }
    }
}