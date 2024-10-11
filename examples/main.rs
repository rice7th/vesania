use vesania::bezier::line::Line;
use vesania::bezier::quadratic::Quadratic;
use vesania::layer::{Layer, Shader};
use vesania::path::Path;
use vesania::render::{FillRule, Renderer};
use vesania::shape::Shape;
use vesania::bezier::Bezier;
use glam::Vec2;
use rgb::Rgba;

#[derive(Debug)]
struct Mat {}
impl Shader for Mat {
    fn fill(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Rgba<f32> {
        return Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

fn main() {
    let mut my_canvas = Canvas::new(200, 200);
    my_canvas.fill_with(Rgba::from((255, 255, 255, 255)));

    let quad = Quadratic::new([10.0, 10.0].into(), [100.0, 100.0].into(), [150.0, 80.0].into());
    let path = Path::new(vec![Box::new(quad)]);

    let my_material = Mat{};

    let rend = Renderer::new(path, Vec2::from([200., 200.]), FillRule::EvenOdd, &my_material);
    let img = rend.render();

    for (i, pix) in img.coverage.iter().enumerate() {
        if *pix > 0.0 {
            *my_canvas.pixel_at_index(i) = 0xFF0000FF;
        }
    }

    my_canvas.write_to_png("out.png").unwrap();
}

pub struct Canvas {
    size: (u16, u16), // 32k by 32k is quite big!
    buffer: Vec<u32>,
}

impl<'pix> Canvas {
    pub fn new(w: u16, h: u16) -> Canvas {
        return Canvas {
            size: (w, h),
            buffer: vec![0u32; (w * h).into()]
        }
    }

    pub fn write_to_png<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), lodepng::Error> {
        self.to_be();
        lodepng::encode32_file(path, &self.buffer, self.size.0.into(), self.size.1.into())
    }

    pub fn pixel_at(&mut self, x: u16, y: u16) -> &mut u32 {
        return self.buffer.get_mut((y * self.size.0 + x) as usize).unwrap()
    }

    pub fn pixel_at_index(&mut self, i: usize) -> &mut u32 {
        return self.buffer.get_mut(i).unwrap()
    }

    pub fn fill_with(&mut self, col: Rgba<u8>) {
        let col = unsafe {
            std::mem::transmute::<Rgba<u8>, u32>(col)
        };
        self.buffer.iter_mut()
            .for_each(|pix| *pix = col);
    }

    pub fn to_be(&mut self) {
        self.buffer.iter_mut()
            .for_each(|pix| *pix = pix.to_be());
    }
}