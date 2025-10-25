//use vulkano::device::Device;
//use vulkano::format::Format;
//use vulkano::render_pass::{RenderPass, Framebuffer, AttachmentLoadOp, AttachmentStoreOp};
//use vulkano::Validated;
//use vulkano::ValidationError;
//use vulkano::VulkanError;
//use vulkano::command_buffer::{SubpassEndInfo, SubpassContents, SubpassBeginInfo, PrimaryAutoCommandBuffer, RenderPassBeginInfo, AutoCommandBufferBuilder, RenderingInfo, RenderingAttachmentInfo};
//use vulkano::pipeline::GraphicsPipeline;
//use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexBuffersCollection};
//use vulkano::buffer::Subbuffer;
//use std::sync::Arc;
//
//use crate::pipeline::gfxpipeline::Material;

//pub fn begin_renderpass(
//    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//    framebuffer: &Arc<Framebuffer>,
//    pipeline: Arc<GraphicsPipeline>)
//{
//    builder
//        .begin_render_pass(
//            RenderPassBeginInfo {
//                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
//                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
//            },
//            SubpassBeginInfo {
//                contents: SubpassContents::Inline,
//                ..Default::default()
//            },
//        ).expect("Couldn't create renderpass");
//        //.bind_pipeline_graphics(pipeline.clone())
//        //.expect("Failed to bind GraphicsPipeline");
//}

//pub fn basic_draw_call<T>(
//    builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//    vertex_buffer: &Subbuffer<[T]>,
//    material: &Material
//    )
//    -> Result<(), Validated<VulkanError>>
//where T: Vertex,
//{
//    let count = vertex_buffer.size();
//
//    let builder = builder.bind_pipeline_graphics(material.gfxpipeline.clone())?;
//    let builder = builder.bind_vertex_buffers(0, vertex_buffer.clone())?;
//    let builder = builder.draw(count as u32, 1, 0, 0)?;
//    Ok(())
//}
//pub fn end_renderpass(builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
//{
//    builder
//    .end_render_pass(SubpassEndInfo::default())
//    .expect("Couldn't end renderpass");
//}
//
//pub fn create_renderpass(device: Arc<Device>) -> Result<Arc<RenderPass>, Validated<VulkanError>>
//{
//    vulkano::single_pass_renderpass!(
//        device.clone(),
//        attachments: {
//            color: {
//            format: Format::R8G8B8A8_UNORM,
//            samples: 1,
//            load_op: Clear,
//            store_op: Store,
//        },
//    },
//        pass: {
//            color: [color],
//            depth_stencil: {},
//        },
//    )
//}
