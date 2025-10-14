pub use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
pub use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
pub use vulkano::memory::allocator::StandardMemoryAllocator;
pub use vulkano::buffer::Subbuffer;
pub use vulkano::command_buffer::*;
pub use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
pub use vulkano::device::{Device, Queue};
pub use vulkano::pipeline::graphics::vertex_input::Vertex;
pub use vulkano::Validated;
pub use vulkano::pipeline::graphics::*;
pub use vulkano::pipeline::graphics::color_blend::*;
pub use vulkano::pipeline::graphics::multisample::*;
pub use vulkano::pipeline::graphics::rasterization::*;
pub use vulkano::pipeline::graphics::viewport::*;
pub use vulkano::pipeline::graphics::input_assembly::*;
pub use vulkano::pipeline::graphics::vertex_input::*;
pub use vulkano::pipeline::layout::*;
pub use vulkano::pipeline::*;
pub use vulkano::render_pass::*;
pub use vulkano::device::*;
pub use vulkano::shader::*;
pub use vulkano::image::Image;
pub use vulkano::image::view::*;
pub use vulkano::sync::*;
pub use vulkano::*;
pub use vulkano::VulkanError;
//pub use shaderc::*;
pub use std::path::*;
pub use std::sync::*;

#[derive(Debug, thiserror::Error)]
pub enum RendererError
{
    #[error("Command buffer execution failed: {0}")]
    CommandExec(#[from] vulkano::command_buffer::CommandBufferExecError),
    #[error("Error with commandbuffer: {0}")]
    VulkanError(#[from] vulkano::Validated<VulkanError>),
    //#[error("Shader error: {0}")]
    //ShaderError(#[from] shaderc::Error)
}
