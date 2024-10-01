use glam::{Vec2, Vec4Swizzles};

use crate::{bezier::Bezier, shape::Shape};

#[derive(Debug)]
pub struct Path {
    data: Vec<Box<dyn Bezier>>
}

impl Path {
    pub fn new(path: Vec<Box<dyn Bezier>>) -> Path {
        return Path { data: path }
    }
    
    pub fn get_intersections(&self, p: Vec2) -> Vec<f32> {
        let mut intersections = Vec::new();
        for element in &self.data {
            let bounds = element.bb();
            if p.y > bounds.wy().min_element()
            && p.y < bounds.wy().max_element() {
                // Our ray at this height hits this element.
                intersections.extend(element.intersections(p));
            }
        }
        return intersections;
    }
}