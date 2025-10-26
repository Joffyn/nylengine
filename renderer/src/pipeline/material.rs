use std::collections::HashSet;
use vulkano::pipeline::graphics::*;
use vulkano::pipeline::graphics::color_blend::*;
use vulkano::pipeline::graphics::multisample::*;
use vulkano::pipeline::graphics::rasterization::*;
use vulkano::pipeline::graphics::viewport::*;
use vulkano::pipeline::graphics::input_assembly::*;
use vulkano::pipeline::graphics::vertex_input::*;
use vulkano::pipeline::layout::*;
use vulkano::pipeline::DynamicState;
use vulkano::pipeline::*;
use vulkano::device::*;
use vulkano::shader::*;
use vulkano::*;
use std::sync::*;

use std::fmt::Display;
use std::hash::RandomState;
use foldhash::HashSetExt;
use vulkano::pipeline::graphics::subpass::PipelineRenderingCreateInfo;
use crate::pipeline::app::{App, RenderContext, GFX_CONTEXT};
use crate::pipeline::vertex::TestVertex;

pub struct Material
{
    pub gfxpipeline: Arc<GraphicsPipeline>
}
impl Material
{
    //Proper error handling needd
    pub fn new(
        vertex_shader: &Arc<ShaderModule>,
        fragment_shader: &Arc<ShaderModule>)
        -> Result<Material, Validated<VulkanError>>
    {
        let vs_entry = vertex_shader.entry_point("main").unwrap();
        let fs_entry = fragment_shader.entry_point("main").unwrap();

        let vertex_input_state =
            TestVertex::per_vertex()
                .definition(&vs_entry)?;

        let stages = [PipelineShaderStageCreateInfo::new(vs_entry), PipelineShaderStageCreateInfo::new(fs_entry)];

        let gfx = GFX_CONTEXT.read().unwrap();
        let gfx = gfx.as_ref().unwrap();

        let layout = PipelineLayout::new(
            gfx.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(gfx.device.clone()).unwrap())?;


        let subpass = PipelineRenderingCreateInfo
        {
            color_attachment_formats: [Some(gfx.swapchain.image_format())].to_vec(),
            ..PipelineRenderingCreateInfo::default()
        };

        use foldhash::HashSet;
        let mut dynamic_state: HashSet<DynamicState> = HashSet::new();
        dynamic_state.insert(DynamicState::Viewport);

        Ok(Self
        {
            gfxpipeline: GraphicsPipeline::new(
                gfx.device.clone(),
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
                    dynamic_state,
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)

                })?,
        })

    }
}

//pub fn create_gfx_pipeline(vs: &Arc<ShaderModule>, ps)
