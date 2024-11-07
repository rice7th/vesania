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

    pub fn get_curve_at_t(&self, t: f32) -> &Box<dyn Bezier> {
        let mut index = (t).floor() as usize;
        if index > self.data.len() - 1 {
            index = self.data.len() - 1
        }
        return &self.data[index];
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
        let t = if t > 0.0 { t - 0.01 } else { t };
        return self.get_curve_at_t(t).t(t.fract());
    }

    fn first_point(&self) -> &Vec2 {
        return self.data.first().unwrap().first_point();
    }

    fn last_point(&self) -> &Vec2 {
        return self.data.last().unwrap().last_point();
    }

    fn derivative(&self, t: f32) -> Vec2 {
        return self.get_curve_at_t(t).derivative(t);
    }

    fn second_derivative(&self, t: f32) -> Vec2 {
        return self.get_curve_at_t(t).second_derivative(t);
    }

    fn parallel(&self, dist: f32) -> Vec<Box<dyn Bezier>> {
        todo!("Not yet implemented")
    }

    fn split(&self, t: f32) -> (Box<dyn Bezier>, Box<dyn Bezier>) {
        return self.get_curve_at_t(t).split(t);
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

    fn fix(&self) -> Vec<Box<dyn Bezier>> {
        let mut new_path = vec![];
        for curve in self.data.iter() {
            new_path.extend(curve.fix());
        }
        return new_path;
    }
}