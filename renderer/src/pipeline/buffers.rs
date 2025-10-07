use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
    SubpassContents, PrimaryAutoCommandBuffer
};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::device::{Device, Queue};

use crate::pipeline::vertex::TestVertex;


pub fn create_vertex_buffer(malloc: Arc<StandardMemoryAllocator>, verts: Vec<TestVertex>) -> Subbuffer<[TestVertex]>
{
    Buffer::from_iter(
    malloc.clone(),
    BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
    AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    ..Default::default() },
    verts).expect("Couldn't create vertex buffer")
}

pub fn begin_commandbuffer_recording(queue: Arc<Queue>, device: Arc<Device>) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>
{

    let cba = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    AutoCommandBufferBuilder::primary(
        &cba,
        queue.clone().queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Couldn't create commandbuffer builder")
}

