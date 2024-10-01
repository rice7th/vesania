use glam::{Vec2, Vec4};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rgb::Rgba;

/// # Layer
/// A Layer is a simple structure that holds
/// a buffer that contains a list of coverage
/// values for each pixel, and a material `M`,
/// responsible for painting each pixel.
#[derive(Debug)]
pub struct Layer<'mat, M> 
where
    M: Shader
{
    pub size: Vec2,
    pub coverage: Vec<f32>,
    pub material: &'mat M,
}

impl<'mat, M> Layer<'mat, M> where M: Shader {
    pub fn new(size: Vec2, material: &M) -> Layer<M> {
        return Layer {
            size,
            material,
            coverage: vec![0.0; (size.x * size.y) as usize]
        };
    }
}

impl<'mat, M> Layer<'mat, M> where M: Shader {
    pub fn paint(&self) -> Rgba<f32> {
        todo!("Implement painting wrapper for each pixel");
    }
}

// DUH
/// # Shader
/// A simple interface for designing custom
/// painting operations.
/// 
/// It is analog to fragment shaders in OpenGL
/// or Vulkan, but on CPU. This allows for a
/// limitless amount of flexibility when choosing
/// any fill option, instead of sticking to the
/// classic Solid-Gradient-Texture format.
/// 
/// For example, a simple Circle SDF can be easily
/// done like this:
/// ```rust
/// pub struct Circle {
///     radius: f32,
///     color: Rgba<f32>
/// }
/// 
/// impl Shader for Circle {
///     fn fill(&self, x: f32, y: f32, w: f32, h: f32) -> Rgba<f32> {
///         if f32::hypot(x, y) < self.radius {
///             return self.color;
///         } else {
///             return Rgba::from([0.0, 0.0, 0.0, 0.0]);
///         }        
///     }
/// }
/// ```
/// The fields of the struct are analogous to OpenGL's
/// `uniforms` or attributes and can of course store any
/// type of data.
pub trait Shader {
    fn fill(&self, _x: f32, _y: f32, _w: f32, _h: f32) -> Rgba<f32>;
}