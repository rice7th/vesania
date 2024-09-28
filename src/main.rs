use std::path::Path;

use bezier::Bezier;
use glam::Vec2;
use rgb::Rgba;

mod bezier;
mod shape;
mod path;
mod layer;

fn main() {
    let mut my_canvas = Canvas::new(100, 100);
    my_canvas.fill_with(Rgba::from((255, 255, 255, 255)));

    let my_curve = bezier::quadratic::Quadratic::new(Vec2::new(1., 1.), Vec2::new(6., 9.), Vec2::new(9., 4.0));
    let my_line  = bezier::line::Line::new(Vec2::new(1., 1.), Vec2::new(6., 9.));

    for a in 1..5 {
        let coords = my_curve.t(1.0 / (a as f32));
        dbg!(coords);
        let pixel = my_canvas.pixel_at((coords.x as u16) * 10, (coords.y as u16) * 10);
        *pixel = 0x00FF00FFu32;
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

    pub fn write_to_png<P: AsRef<Path>>(&mut self, path: P) -> Result<(), lodepng::Error> {
        self.to_be();
        lodepng::encode32_file(path, &self.buffer, self.size.0.into(), self.size.1.into())
    }

    pub fn pixel_at(&mut self, x: u16, y: u16) -> &mut u32 {
        return self.buffer.get_mut((y * self.size.0 + x) as usize).unwrap()
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