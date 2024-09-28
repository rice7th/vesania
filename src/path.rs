use glam::Vec2;

use crate::{bezier::Bezier, shape::Shape};

pub struct Path<B: Bezier> {
    data: Vec<B>
}

impl<B: Bezier> Shape for Path<B> {
    fn winding_number(p: Vec2) -> i8 {
        todo!()
    }
}