//! # Common fills
//! Common fill like Solid color, linear and radial
//! gradient, texture etc. so the user doesn't need
//! to implement them again and again.

use rgb::*;
use crate::layer::Shader;

/// # Solid Fill
/// A simple solid color fill.
#[derive(Debug)]
pub struct Solid {
    col: Rgba<f32>
}

impl Solid {
    pub fn new(col: [f32; 4]) -> Solid {
        return Solid {
            col: col.into()
        }
    }
}

impl Shader for Solid {
    fn fill(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Rgba<f32> {
        return self.col;
    }
}