use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

mod threading;
pub mod renderer;
mod pipline;
mod renderpass;
mod camera;
mod metrix;
mod shader;
mod teapot;
mod quad;
mod gltf;
