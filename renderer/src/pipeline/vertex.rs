use std::ops::Deref;
use std::sync::Arc;
use bytemuck::Pod;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use crate::pipeline::app::GFX_CONTEXT;

#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct TestVertex {
    #[format(R32G32B32_SFLOAT)]
    pub pos: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

pub struct IndexBuffer
{
    inner: Subbuffer<[u16]>,
}
impl IndexBuffer
{
    pub fn new(indices: Vec<u16>) -> IndexBuffer
    {
        let gfx = GFX_CONTEXT.read().unwrap();
        let gfx = gfx.as_ref().unwrap();
        IndexBuffer
        {
            inner: Buffer::from_iter(
                 gfx.std_malloc.clone(),
                BufferCreateInfo { usage: BufferUsage::INDEX_BUFFER, ..Default::default()},
                AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default() },
                indices).expect("Couldn't create vertex buffer"),
        }
    }
}
impl Deref for IndexBuffer
{
    type Target = Subbuffer<[u16]>;
    fn deref(&self) -> &Self::Target { &self.inner }
}


pub struct VertexBuffer
{
    inner: Subbuffer<[TestVertex]>
}
impl VertexBuffer
{
    pub fn new(verts: Vec<TestVertex>) -> VertexBuffer
    {
        let gfx = GFX_CONTEXT.read().unwrap();
        let gfx = gfx.as_ref().unwrap();
        VertexBuffer
        {
            inner: Buffer::from_iter(
                gfx.std_malloc.clone(),
                BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
                AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default() },
                verts).expect("Couldn't create vertex buffer")
        }
    }

}
impl Deref for VertexBuffer
{
    type Target = Subbuffer<[TestVertex]>;
    fn deref(&self) -> &Self::Target { &self.inner }
}

