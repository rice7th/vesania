use glam::{Vec2, Vec4};
use rgb::Rgba;

pub struct Layer<F, C> 
where
    F: Fn(Vec4, C) -> Rgba<f32>
{
    size: Vec2,
    coverage: Vec<f32>,
    fill: F,
    _marker: std::marker::PhantomData<fn(C)>,
}

impl<F, C> Layer<F, C> where F: Fn(Vec4, C) -> Rgba<f32> {

}