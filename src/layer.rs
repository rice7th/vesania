use glam::{Vec2, Vec4};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rgb::Rgba;

/// # Layer
/// A Layer is a simple structure that holds
/// a buffer that contains a list of coverage
/// values for each pixel, and a function
/// `fill` that given four mandatory parameters
/// (x, y of a pixel and width and height of
/// the Layer) and a custom parameter C, containing
/// any other data that the user may want to
/// provide (e.g. gradient stops, noise seeds,
/// textures etc.) runs for each pixel filling
/// it in with some color.
/// 
/// It is analog to fragment shaders in OpenGL
/// or Vulkan, but on CPU. This allows for a
/// limitless amount of flexibility when choosing
/// any fill option, instead of sticking to the
/// classic Solid-Gradient-Texture format.
pub struct Layer<F, C> 
where
    F: Fn(Vec4, C) -> Rgba<f32>
{
    size: Vec2,
    coverage: Vec<f32>,
    material: F,
    _marker: std::marker::PhantomData<fn(C)>,
}

impl<F, C> Layer<F, C> where F: Fn(Vec4, C) -> Rgba<f32> {
    pub fn fill(&self) -> Rgba<f32> {
        // TODO: How do I even approach this problem?
        //(self.material)(x, y, w, h, )
        todo!()
    }
}