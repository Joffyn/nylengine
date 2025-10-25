use std::collections::HashSet;
use vulkano::pipeline::graphics::*;
use vulkano::pipeline::graphics::color_blend::*;
use vulkano::pipeline::graphics::multisample::*;
use vulkano::pipeline::graphics::rasterization::*;
use vulkano::pipeline::graphics::viewport::*;
use vulkano::pipeline::graphics::input_assembly::*;
use vulkano::pipeline::graphics::vertex_input::*;
use vulkano::pipeline::layout::*;
use vulkano::pipeline::*;
use vulkano::render_pass::*;
use vulkano::device::*;
use vulkano::shader::*;
use vulkano::*;
use std::sync::*;

use std::fmt::Display;
use vulkano::pipeline::DynamicState::Viewport;
use vulkano::pipeline::graphics::subpass::PipelineRenderingCreateInfo;
use winit::window::CursorIcon::Default;
use crate::pipeline::app::{App, RenderContext};
//use crate::pipeline::prelude::RendererError::VulkanError;

pub struct Material
{
    pub gfxpipeline: Arc<GraphicsPipeline>
}
impl Material
{
    //Proper error handling needd
    pub fn new<T>(
        vertex_shader: &Arc<ShaderModule>,
        fragment_shader: &Arc<ShaderModule>,
        app: &App) -> Result<Material, Validated<VulkanError>>
    where T : Vertex,
    {
        let vs_entry = vertex_shader.entry_point("main").unwrap();
        let fs_entry = fragment_shader.entry_point("main").unwrap();

        let rcx: &RenderContext = match app.rcx
        {
            Ok(rcx) => rcx,
            Err(e) => return e,
        };

        let vertex_input_state =
            T::per_vertex()
                .definition(&vs_entry)?;

        let stages = [PipelineShaderStageCreateInfo::new(vs_entry), PipelineShaderStageCreateInfo::new(fs_entry)];

        let layout = PipelineLayout::new(
            app.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(app.device.clone()).unwrap())?;


        let subpass = PipelineRenderingCreateInfo
        {
            color_attachment_formats: [Some(rcx.swapchain.image_format())].to_vec(),
            ..PipelineRenderingCreateInfo::default()
        };

        Ok(Self
        {
            gfxpipeline: GraphicsPipeline::new(
                app.device.clone(),
                None,
                GraphicsPipelineCreateInfo
                {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.color_attachment_formats.len() as u32,
                        ColorBlendAttachmentState::default(),
                    )),
                    dynamic_state: &[DynamicState::Viewport],
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)

                })?,
        })

    }
}

//pub fn create_gfx_pipeline(vs: &Arc<ShaderModule>, ps)
