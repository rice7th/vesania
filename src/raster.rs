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


/// # Shadows
/// A list of shadows cast by any segment.
/// Shadows can be positive or negative, 
/// annihilating each other in the process.
#[derive(Clone, Debug)]
pub struct Shadows(Vec<Vec2>);

impl Shadows {
    pub fn new(intervals: Vec<Vec2>) -> Shadows {
        return Shadows(intervals);
    }

    pub fn inner(self) -> Vec<Vec2> {
        return self.0;
    }

    pub fn fuse(self) -> Shadows {
        let mut inter = self.inner();
        inter.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let mut vet = vec![];
        let mut start = inter[0][0];
        let mut end = inter[0][1];
        for i in 0..inter.len() {
            if inter[i][0] <= end {
                end = f32::max(inter[i][1], end);
            } else { // Outside the range
                vet.push([start, end].into());
                start = inter[i][0];
                end = inter[i][1];
            }
        }
        vet.push([start, end].into());
        return Self::new(vet);
    }
}