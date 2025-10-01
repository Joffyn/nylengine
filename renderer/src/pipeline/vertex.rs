use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::buffer::BufferContents;


#[derive(BufferContents, Vertex, Debug, Clone)]
#[repr(C)]
pub struct TestVertex {
    #[format(R32G32_SFLOAT)]
    pub pos: [f32; 2],
}

