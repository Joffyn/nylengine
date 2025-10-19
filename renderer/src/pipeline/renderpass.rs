use std::sync::Arc;
use wgpu::{CommandEncoder, RenderPass, TextureView};
use crate::pipeline::model::Model;
use crate::pipeline::shapes::create_triangle;
//use crate::pipeline::gfxpipeline::Material;

pub fn basic_draw_call(render_pass: &mut RenderPass, model: &Model)
{
render_pass.set_pipeline(model.base_pipeline());
render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
render_pass.draw_indexed(0..model.index_buffer.size() as u32, 0, 0..1); // 2.
}

pub fn main_pass(cmd_encoder: &mut CommandEncoder, texture_view: &TextureView)
{
    //let render_pass = cmd_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //    label: None,
    //    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //        view: &texture_view,
    //        depth_slice: None,
    //        resolve_target: None,
    //        ops: wgpu::Operations {
    //            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    //            store: wgpu::StoreOp::Store,
    //        },
    //    })],
    //    depth_stencil_attachment: None,
    //    timestamp_writes: None,
    //    occlusion_query_set: None,
    //});
    //basic_draw_call(&mut render_pass, create_triangle())
    //


    //drop(render_pass);
}

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
//
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
