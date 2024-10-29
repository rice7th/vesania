use core::f32;

use glam::Vec2;

use crate::{bezier::{Bezier, Direction}, layer::{Layer, Shader}, path::Path, shape::Shape};

#[derive(Debug)]
pub struct Renderer<'mat, M: Shader> {
    rule: FillRule,
    size: Vec2,
    path: Path,
    material: &'mat M
}

impl<'mat, M> Renderer<'mat, M> where M: Shader {
    pub fn new(path: Path, size: Vec2, rule: FillRule, material: &'mat M) -> Renderer<M> {
        return Renderer { path, size, rule, material };
    }

    // TODO: use SIMD and a lot of threads
    // TODO: Split into scanlines
    // NOTE: 
    pub fn render(&self) -> Layer<M> {
        let mut layer = Layer::new(self.size, self.material);
        for (i, pixel) in layer.coverage.iter_mut().enumerate() {
            let p = Vec2::from([i as f32 % layer.size.x, (i as f32 / layer.size.y).floor()]);
            // If a ray hits a point shared between two curves, we decide wether the intersection
            // counts twice (different winding direction) or once (same winding direction).
            let mut last = f32::NAN;
            let raw_inters = self.path.intersections(p);
            let mut inters = Vec::new();
            for i in raw_inters.iter() {
                last;
                let prev = self.path.get_curve_at_t(last.floor() - 0.5).slope(0.5);
                let next = self.path.get_curve_at_t(last.floor() + 0.5).slope(0.5);
                if last == *i || last < (*i + 0.001) || last > (*i - 0.001) { // Or close enough
                    if prev.signum() == next.signum() {
                        continue;
                    }
                }
                last = *i;
                inters.push(last);
            };
            let mut winding = 0;
            for j in inters {
                let point = self.path.t(j).x;
                
                if point <= p.x {
                    match self.rule {
                        FillRule::EvenOdd => winding += 1,
                        FillRule::NonZero => match self.path.get_curve_at_t(j.floor()).direction() {
                            Direction::Up   => winding += 1,
                            Direction::Down => winding -= 1,
                        }
                    }
                }
            }
            // I'm sure there's a better way to do this
            // FIXME: Implement AA
            match self.rule {
                FillRule::EvenOdd => if winding % 2 == 1 { *pixel = 1.0; },
                FillRule::NonZero => if winding != 0 { *pixel = 1.0 }
            }
        }
        return layer;
    }
}

#[derive(Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd
}