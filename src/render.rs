use glam::Vec2;

use crate::{layer::{Layer, Shader}, path::Path};

#[derive(Debug)]
pub struct Renderer<'mat, M: Shader> {
    rule: FillRule,
    size: Vec2,
    path: Path,
    material: &'mat M
}

impl<'mat, M> Renderer<'mat, M> where M: Shader {
    pub fn new(path: Path, size: Vec2, rule: FillRule, material: &'mat M) -> Renderer<M> {
        return Renderer { path, size, rule, material };
    }

    // Todo: use SIMD and a lot of threads
    pub fn render(&self) -> Layer<M> {
        let mut layer = Layer::new(self.size, self.material);
        for (i, pixel) in layer.coverage.iter_mut().enumerate() {
            let p = Vec2::from([i as f32 % layer.size.x, (i as f32 / layer.size.y).floor()]);
            let inters = self.path.get_intersections(p);
            let mut winding = 0;
            for j in inters {
                if j > p.x { winding += 1 }
            }
            dbg!(winding);
            if winding % 2 == 1 { *pixel = 1.0 } // For now let's do full coverage
        }
        return layer;
    }
}

#[derive(Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd
}