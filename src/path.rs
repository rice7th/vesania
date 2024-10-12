use glam::{Vec2, Vec4, Vec4Swizzles};

use crate::{bezier::Bezier, shape::Shape};

#[derive(Debug)]
pub struct Path {
    data: Vec<Box<dyn Bezier>>
}

impl Path {
    pub fn new(path: Vec<Box<dyn Bezier>>) -> Path {
        return Path { data: path }
    }
}

impl Shape for Path {
    fn intersections(&self, p: Vec2) -> Vec<f32> {
        let mut intersections = Vec::new();
        for (index, element) in self.data.iter().enumerate() {
            let bounds = element.bb();
            if p.y > bounds.wy().min_element()
            && p.y < bounds.wy().max_element() {
                // Our ray at this height hits this element.
                let el_int = element.intersections(p)
                    .iter()
                    .map(|i| i + index as f32)
                    .collect::<Vec<f32>>();
                intersections.extend(el_int);
            }
        }
        return intersections;
    }
}

impl Bezier for Path {
    fn t(&self, t: f32) -> Vec2 {
        let index = (t - 0.00001).floor() as usize;
        return self.data[index].t(t.fract());
    }
    
    fn bb(&self) -> Vec4 {
        let mut min = Vec2::INFINITY;
        let mut max = Vec2::NEG_INFINITY;
        for element in &self.data {
            let minmax = element.bb();
            if minmax.x < min.x { min.x = minmax.x }
            if minmax.y < min.y { min.y = minmax.y }
            if minmax.z < max.x { max.x = minmax.z }
            if minmax.w < max.y { max.y = minmax.w }
        }
        return Vec4::from([min.x, min.y, max.x, max.y]);
    }
}