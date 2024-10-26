//! # Common fills
//! Common fill like Solid color, linear and radial
//! gradient, texture etc. so the user doesn't need
//! to implement them again and again.

use rgb::*;
use glam::{Vec2, Vec4};
use crate::layer::Shader;

pub fn mix(a: Vec4, b: Vec4, t: f32) -> Vec4 {
    return (1.0 - t) * a + b*t;
}

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

// Simple, two stop radial gradient fill.
// TODO: Add arbitrary stops
#[derive(Debug)]
pub struct Radial {
    start: Rgba<f32>,
    end: Rgba<f32>,
    center: Vec2,
    scale: f32,
}

impl Radial {
    pub fn new(start: [f32; 4], end: [f32; 4], center: [f32; 2], scale: f32) -> Radial {
        return Radial {
            start: start.into(),
            end: end.into(),
            center: center.into(),
            scale
        }
    }
}

impl Shader for Radial {
    fn fill(&self, x: f32, y: f32, w: f32, h: f32) -> Rgba<f32> {
        use std::mem::transmute;
        let (x, y) = (x / w, y / h);
        let d = f32::hypot(x - self.center.x, y - self.center.y) * (1. / self.scale); // from center basically
        let col = unsafe { // Please find a better way to do this.
            transmute::<Vec4, Rgba<f32>>(mix(
                transmute::<Rgba<f32>, Vec4>(self.start),
                transmute::<Rgba<f32>, Vec4>(self.end),
                d
            ))
        };
        return col;
    }
}