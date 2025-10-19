use std::sync::Arc;
use bytemuck::Pod;
use wgpu::{Buffer, BufferUsages, Device, RenderPipeline, ShaderModule};
use wgpu::hal::{PipelineLayoutDescriptor, PipelineLayoutFlags};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::pipeline::material::{Material, MaterialInstance};
use crate::pipeline::vertex::Vertex;

pub struct Model
{
    material_instance: Arc<MaterialInstance>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Model
{
    pub fn new<T>(device: &Device, material_instance: Arc<MaterialInstance>, verts: &[T], indices: &[u16]) -> Model
    where T: Vertex + Pod,
    {
        Model
        {
            material_instance: material_instance.clone(),
            vertex_buffer: device.create_buffer_init(
                &BufferInitDescriptor
                {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(verts),
                    usage: BufferUsages::VERTEX,
                }
            ),
            index_buffer: device.create_buffer_init(
                &BufferInitDescriptor
                {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: BufferUsages::INDEX,
                }
            )

        }
    }
    pub fn base_pipeline(&self) -> &RenderPipeline
    {
        &self.material_instance.base_material.render_pipeline
    }
}
