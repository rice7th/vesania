use core::f32;
use std::{mem::swap, ops::{Add, BitXor, Mul, Sub}, simd::f32x4};

use glam::{Vec2, Vec4};

use crate::{bezier::{line::Line, Bezier, Direction}, layer::{Layer, Shader}, path::Path, shape::{GridSegments, Shape}};

#[derive(Debug)]
pub struct Renderer<'mat, M: Shader> {
    rule: FillRule,
    size: Vec2,
    path: Path,
    material: &'mat M
}

// TODO: RGB, BGR, VRGB, VBGR
// Note: the steam deck uses VBRG for some reason.
// Note: Pentile displays are almost useless since most of them
// are very high dpi.
// https://geometrian.com/resources/subpixelzoo/
// https://en.wikipedia.org/wiki/PenTile_matrix_family#PenTile_RGBG
impl<'mat, M> Renderer<'mat, M> where M: Shader {
    pub fn new(path: Path, size: Vec2, rule: FillRule, material: &'mat M) -> Renderer<'mat, M> {
        return Renderer { path, size, rule, material };
    }

    /// # Split
    /// Step one of the new AA pipeline. Splits
    /// a path into a series of segments that are
    /// contained in less than one pixel.
    /// <br>
    /// **NOTE**: This currently works only with lines.
    /// In the future I will implement some sort of
    /// interface for each bezier that returns its intersections
    /// with the pixel grid in a parallel way.
    //  TODO: convert path elements to lines?
    pub fn split(&self) -> GridSegments {
        let mut grid = vec![vec![vec![]; self.size.y as usize]; self.size.x as usize];
        self.path.grid_intersections(&mut grid);
        return grid;
    }

    // TODO: use SIMD and a lot of threads
    // TODO: Split into scanlines
    // NOTE: 
    pub fn render(&self) -> Layer<M> { // i % w == x; i / y == y
        let mut layer = Layer::new(self.size, self.material);
        // Step 1: Quantization
        let segments = self.split();

        for x in segments {
            for y in x {
                if !y.is_empty() {
                    for z in y {
                        println!("({}, {}),", z.first_point().x, z.first_point().y);
                        println!("({}, {}),", z.last_point().x, z.last_point().y);
                    }
                }
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