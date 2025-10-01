use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::buffer::Subbuffer;

use crate::pipeline::vertex::TestVertex;

struct VertexBuffer
{

}

pub fn create_vertex_buffer(malloc: Arc<StandardMemoryAllocator>, verts: Vec<TestVertex>) -> Subbuffer<[TestVertex]>
{
    Buffer::from_iter(
    malloc.clone(),
    BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
    AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    ..Default::default() },
    verts).expect("Couldn't create vertex buffer")
}


