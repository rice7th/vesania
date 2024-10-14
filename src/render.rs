use glam::Vec2;

use crate::{layer::{Layer, Shader}, path::Path, shape::Shape, bezier::Bezier};

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

    // TODO: use SIMD and a lot of threads
    // TODO: Split into scanlines
    pub fn render(&self) -> Layer<M> {
        let mut layer = Layer::new(self.size, self.material);
        for (i, pixel) in layer.coverage.iter_mut().enumerate() {
            let p = Vec2::from([i as f32 % layer.size.x, (i as f32 / layer.size.y).floor()]);
            let inters = self.path.intersections(p);
            let mut winding = 0;
            for j in inters {
                dbg!(j);
                if self.path.t(j).x > p.x {
                    match self.rule {
                        FillRule::EvenOdd => winding += 1,
                        FillRule::NonZero => if self.path.slope(j) < 0.0 { // Use Epsilon
                            winding -= 1;
                        } else {
                            winding += 1;
                        }
                    }                    
                }
                
            }
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