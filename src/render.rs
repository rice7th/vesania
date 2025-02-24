use core::f32;
use std::{mem::swap, ops::{Add, BitXor}};

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
    pub fn new(path: Path, size: Vec2, rule: FillRule, material: &'mat M) -> Renderer<'mat, M> {
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


#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: f32,
    end: f32
}

impl Span {
    pub fn new(start: f32, end: f32) -> Span {
        return Span { start, end };
    }

    pub fn direction(&self) -> f32 {
        if self.start < self.end {
            return 1.0;
        } else {
            return -1.0;
        }
    }
}

impl Add for Span {
    type Output = Vec<Span>;
    // This is wrong at the moment
    // We need to first check if we're overlapping
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = if self.direction() == -1.0 { // Swap if direction is negative
            Span::new(self.end, self.start)
        } else {
            self
        };

        let rhs = if rhs.direction() == -1.0 { // Swap if direction is negative
            Span::new(rhs.end, rhs.start)
        } else {
            rhs
        };

        // Check if we're overlapping

        return vec![Span::new(f32::min(lhs.start, rhs.start), f32::max(lhs.end, rhs.end))];
    }
}

impl BitXor for Span {
    type Output = Vec<Span>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut arr = [self.start, self.end, rhs.start, rhs.end];
        if arr[0] > arr[2] { arr.swap(0, 2) }
        if arr[1] > arr[3] { arr.swap(1, 3) }
        if arr[0] > arr[1] { arr.swap(0, 1) }
        if arr[2] > arr[3] { arr.swap(2, 3) }
        if arr[1] > arr[2] { arr.swap(1, 2) }

        return vec![Span::new(arr[0], arr[1]), Span::new(arr[2], arr[3])];
    }
}