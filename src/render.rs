use glam::Vec2;

use crate::{layer::{Layer, Shader}, path::Path};

pub struct Renderer<M: Shader> {
    rule: FillRule,
    size: Vec2,
    path: Path,
    material: M
}

impl<M> Renderer<M> where M: Shader {
    pub fn new(path: Path, size: Vec2, rule: FillRule, material: M) -> Renderer<M> {
        return Renderer { path, size, rule, material };
    }

    pub fn render() -> Layer<M> {
        todo!()
    }
}


pub enum FillRule {
    NonZero,
    EvenOdd
}