use vesania::bezier::line::Line;
use vesania::bezier::quadratic::Quadratic;
use vesania::fills;
use vesania::layer::{Image, Layer, Shader};
use vesania::path::Path;
use vesania::render::{FillRule, Renderer};
use vesania::shape::Shape;
use vesania::bezier::Bezier;
use glam::Vec2;
use rgb::{Pixel, Rgba};

fn main() {
    let mut my_canvas = Canvas::new(3000, 3000);
    my_canvas.fill_with(Rgba::from((255, 255, 255, 255)));

    let quad1 = Quadratic::new([400.0, 100.0].into(), [100.0, 100.0].into(), [100.0, 400.0].into());
    let quad2 = Quadratic::new([100.0, 400.0].into(), [100.0, 700.0].into(), [400.0, 700.0].into());
    let quad3 = Quadratic::new([400.0, 700.0].into(), [700.0, 700.0].into(), [700.0, 400.0].into());
    let quad4 = Quadratic::new([700.0, 400.0].into(), [700.0, 100.0].into(), [400.0, 100.0].into());

    let quad5 = Quadratic::new([50.0, 10.0].into(), [20.0, 10.0].into(), [20.0, 40.0].into());
    let quad6 = Quadratic::new([20.0, 40.0].into(), [20.0, 70.0].into(), [50.0, 70.0].into());
    let quad7 = Quadratic::new([50.0, 70.0].into(), [80.0, 70.0].into(), [80.0, 40.0].into());
    let quad8 = Quadratic::new([80.0, 40.0].into(), [80.0, 10.0].into(), [50.0, 10.0].into());

    let quad = Quadratic::new([10.0, 10.0].into(), [150.0, 400.0].into(), [290.0, 10.0].into());

    let path = Path::new(vec![
        Box::new(quad1),
        //Box::new(quad2),
        //Box::new(quad3),
        //Box::new(quad4),
        //Box::new(quad5), Box::new(quad6), Box::new(quad7), Box::new(quad8),
    ]);

    //let my_material = fills::Radial::new([0.1, 1.0, 1.0, 1.0], [0.4, 1.0, 0.2, 1.0], [0.1, 0.1], 0.2);
    let my_material = fills::Radial::new([1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.1, 0.1], 0.2);

    let rend = Renderer::new(path, Vec2::from([3000., 3000.]), FillRule::NonZero, &my_material);
    let img = rend.render();

    my_canvas.image(img.paint());
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
            buffer: vec![0u32; w as usize * h as usize]
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

    pub fn image(&mut self, img: Image) {
        for (i, pix) in self.buffer.iter_mut().enumerate() {
            *pix = unsafe {
                std::mem::transmute::<Rgba<u8>, u32>(img.pixels.get(i).unwrap_or(&Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }).map(|col| (col * 255.0) as u8)).to_be()
            }
        }
    }
}