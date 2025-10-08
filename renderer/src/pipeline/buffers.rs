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
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::Validated;
use vulkano::VulkanError;

use crate::pipeline::vertex::TestVertex;


pub fn create_vertex_buffer<T>(malloc: Arc<StandardMemoryAllocator>, verts: Vec<T>) -> Subbuffer<[T]>
where T: Vertex,
{
    Buffer::from_iter(
    malloc.clone(),
    BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
    AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    ..Default::default() },
    verts).expect("Couldn't create vertex buffer")
}

pub fn begin_commandbuffer_recording(queue: Arc<Queue>, device: Arc<Device>)
-> Result<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, Validated<VulkanError>>
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
}

//pub fn end_commandbuffer_recording(cmdbuffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, gfxpipeline: ())
//-> Result<Arc<PrimaryAutoCommandBuffer>, Validated<VulkanError>>
//{
//
//}

