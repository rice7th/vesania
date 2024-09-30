use glam::Vec2;

use crate::{bezier::Bezier, shape::Shape};

pub struct Path {
    data: Vec<Box<dyn Bezier>>
}