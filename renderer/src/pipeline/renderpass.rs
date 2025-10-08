use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::render_pass::{RenderPass, Framebuffer};
use vulkano::Validated;
use vulkano::ValidationError;
use vulkano::VulkanError;
use vulkano::command_buffer::{SubpassEndInfo, SubpassContents, SubpassBeginInfo,
    PrimaryAutoCommandBuffer, RenderPassBeginInfo, AutoCommandBufferBuilder};
use std::sync::Arc;

//pub fn begin_renderpass(builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, framebuffer: &Arc<Framebuffer>)
//-> Result<&mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, Box<ValidationError>>
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
//        )
//        .unwrap()
//        .end_render_pass(SubpassEndInfo::default())?;
//
//
//
//}

#[allow(unused)]
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
