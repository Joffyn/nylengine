use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::render_pass::RenderPass;
use vulkano::Validated;
use vulkano::VulkanError;
use std::sync::Arc;

#[allow(unused)]
pub fn create_render_pass(device: Arc<Device>) -> Result<Arc<RenderPass>, Validated<VulkanError>>
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
