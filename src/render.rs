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
        let mut inters = Vec::new();
        for (index, pixel) in layer.coverage.iter_mut().enumerate() {
            let p = Vec2::from([index as f32 % layer.size.x, (index as f32 / layer.size.y).floor()]);
            if index % self.size.x as usize == 0 {
                inters.clear();
                // If a ray hits a point shared between two curves, we decide wether the intersection
                // counts twice (different winding direction) or once (same winding direction).
                let mut last = f32::NAN;
                let raw_inters = self.path.intersections(p);
                for curr in raw_inters.iter() {
                    if last == *curr || last <= (*curr + f32::EPSILON) && last >= (*curr - f32::EPSILON) { // Or close enough
                        let prev = self.path.get_curve_at_t(last.floor()).direction();
                        let next = self.path.get_curve_at_t(curr.floor()).direction();
                        if prev == next {
                            continue;
                        }
                    }
                    last = *curr;
                    inters.push((*curr, self.path.get_curve_at_t(*curr).direction()));
                };
            }            

            let mut winding = 0;
            for (int, dir) in &inters {
                let int = self.path.t(*int).x;
                if int <= p.x {
                    match self.rule {
                        FillRule::EvenOdd => winding += 1,
                        FillRule::NonZero => match dir {
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