use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait Vertex
{
    fn desc<'a>() -> VertexBufferLayout<'static>;
}


#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub struct TestVertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl TestVertex
{
    const ATTRIBS: [VertexAttribute; 2]
    = vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}
impl Vertex for TestVertex
{
    fn desc() -> VertexBufferLayout<'static>
    {
        VertexBufferLayout
        {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }

    }
}

