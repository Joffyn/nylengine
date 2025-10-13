use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::render_pass::{RenderPass, Framebuffer};
use vulkano::Validated;
use vulkano::ValidationError;
use vulkano::VulkanError;
use vulkano::command_buffer::{SubpassEndInfo, SubpassContents, SubpassBeginInfo,
    PrimaryAutoCommandBuffer, RenderPassBeginInfo, AutoCommandBufferBuilder};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexBuffersCollection};
use vulkano::buffer::Subbuffer;
use std::sync::Arc;

pub fn begin_renderpass(
    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, 
    framebuffer: &Arc<Framebuffer>,
    pipeline: Arc<GraphicsPipeline>)
{
    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassBeginInfo {
                contents: SubpassContents::Inline,
                ..Default::default()
            },
        ).expect("Couldn't create renderpass");
        //.bind_pipeline_graphics(pipeline.clone())
        //.expect("Failed to bind GraphicsPipeline");
}

pub fn basic_draw_call<T>(
    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, 
    vertex_buffer: impl VertexBuffersCollection + std::clone::Clone,
    vertex_count: u32)
where T: Vertex,
{
    builder.bind_vertex_buffers(0, vertex_buffer.clone())
    .expect("Couldn't bind vertex buffer")
    .draw(vertex_count, 1, 0, 0)
    .expect("Couldn't issue draw call");
}
pub fn end_renderpass(builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
{
    builder
    .end_render_pass(SubpassEndInfo::default())
    .expect("Couldn't end renderpass");
}

pub fn create_renderpass(device: Arc<Device>) -> Result<Arc<RenderPass>, Validated<VulkanError>>
{
    vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
            format: Format::R8G8B8A8_UNORM,
            samples: 1,
            load_op: Clear,
            store_op: Store,
        },
    },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
}
