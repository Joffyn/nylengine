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

pub struct Material
{
    gfxpipeline: Arc<GraphicsPipeline>
}
impl Material
{
    //Proper error handling needd
    pub fn new<T>(
        vertex_shader: &Arc<ShaderModule>,
        fragment_shader: &Arc<ShaderModule>,
        device: Arc<Device>,
        renderpass: Arc<RenderPass>) -> Result<Material, Box<dyn std::error::Error>>
    where T : Vertex,
    {
        let vs_entry = vertex_shader.entry_point("main").unwrap();
        let fs_entry = fragment_shader.entry_point("main").unwrap();

        
        let vertex_input_state = 
            T::per_vertex()
            .definition(&vs_entry.info().input_interface)?;

        let stages = [PipelineShaderStageCreateInfo::new(vs_entry), PipelineShaderStageCreateInfo::new(fs_entry)];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())?)?;


        let subpass = Subpass::from(renderpass.clone(), 0).unwrap();

        let viewport = Viewport 
        {
            offset: [0.0, 0.0],
            extent: [1024.0, 1024.0],
            depth_range: 0.0..=1.0,
        };

        Ok(Self
        { 
            gfxpipeline: GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo
            {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState
                    {
                        viewports: [viewport].into_iter().collect(),
                        ..Default::default()
                    }),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)

            })?,
        })

    }
}

//pub fn create_gfx_pipeline(vs: &Arc<ShaderModule>, ps)
