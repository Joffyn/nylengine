use std::iter::TrustedRandomAccessNoCoerce;
use std::ops::Deref;
use std::sync::Arc;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};

#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct TestVertex {
    #[format(R32B32G32_SFLOAT)]
    pub pos: [f32; 3],
    #[format(R32B32G32_SFLOAT)]
    pub color: [f32; 3],
}

pub struct IndexBuffer
{
    inner: Subbuffer<[u16]>,
    amount: u32,
}
impl IndexBuffer
{
    pub fn new(malloc: Arc<StandardMemoryAllocator>, indices: &[u16]) -> IndexBuffer
    {

        IndexBuffer
        {
            inner: Buffer::from_iter(
                malloc.clone(),
                BufferCreateInfo { usage: BufferUsage::INDEX_BUFFER, ..Default::default()},
                AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default() },
                *indices).expect("Couldn't create vertex buffer"),
            amount: indices.iter().size() as u32
        }
    }
}
impl Deref for IndexBuffer
{
    type Target = Subbuffer<[u16]>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
pub struct VertexBuffer<T>
where T: Vertex
{
    inner: Subbuffer<[T]>
}
impl<T> VertexBuffer<T>
{
    pub fn new(malloc: Arc<StandardMemoryAllocator>, verts: &Vec<T>) -> VertexBuffer<T>
    {
        VertexBuffer
        {
            inner: Buffer::from_iter(
                malloc.clone(),
                BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
                AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default() },
                verts).expect("Couldn't create vertex buffer")
        }
    }

}
impl<T> Deref for VertexBuffer<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

