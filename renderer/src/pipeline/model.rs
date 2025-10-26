use std::collections::HashMap;
use std::fs::read;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use shaderc::ShaderKind;
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::shader::ShaderModule;
use crate::pipeline::app::GFX_CONTEXT;
use crate::pipeline::material::Material;
use crate::pipeline::mesh_data::MeshData;
use crate::pipeline::shaderloading::{compile_shader, load_shader};
use crate::pipeline::vertex::TestVertex;


static MODELS : Lazy<RwLock<Vec<Model>>> = Lazy::new(|| {
    let mut v = Vec::new();
    RwLock::new(v)
});
//Gives index of last pushed model

pub fn draw_all_models(builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
{
    let drawables = MODELS.read().unwrap();

    for drawable in drawables.iter()
    {
        drawable.draw(builder);
    }
}
pub fn create_model(mesh_data: Arc<MeshData>, material: Arc<Material>) -> usize
{
    let model = Model
    {
        mesh_data,
        material
    };
    MODELS.write().unwrap().push(model);
    MODELS.read().unwrap().len() - 1
}

//Actual real model
pub struct Model
{
    pub mesh_data: Arc<MeshData>,
    pub material: Arc<Material>
}

impl Model
{
    pub fn draw(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
    {
        builder
            .bind_pipeline_graphics(self.material.gfxpipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, self.mesh_data.vertex_buffer.clone())
            .unwrap()
            .bind_index_buffer(self.mesh_data.index_buffer.clone())
            .unwrap();

        unsafe { builder.draw(
            self.mesh_data.vertex_buffer.len() as u32,
            1,
            0,
            0)};
    }
}

const TRIANGLE_VERTICES: &[TestVertex] = &[
    TestVertex { pos: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    TestVertex { pos: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    TestVertex { pos: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    TestVertex { pos: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    TestVertex { pos: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
pub fn create_triangle() -> usize
{
    let vertex_path = Path::new("shaders/vertex.spv");
    let pixel_path = Path::new("shaders/pixel.spv");

    if !Path::exists(vertex_path) || !Path::exists(pixel_path)
    {
        compile_shader(ShaderKind::Vertex, Path::new("shaders/vertex.hlsl")).unwrap();
        compile_shader(ShaderKind::Fragment, Path::new("shaders/pixel.hlsl")).unwrap();
    }

    let gfx = GFX_CONTEXT.read().unwrap();
    let gfx = gfx.as_ref().unwrap();

    //Copying slow
    let vertex_binary: Vec<u32> = read(vertex_path)
        .unwrap()
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    //Copying slow
    let pixel_binary: Vec<u32> = read(pixel_path)
        .unwrap()
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    let vertex_mod = load_shader(vertex_binary, gfx.device.clone()).unwrap();
    let pixel_mod = load_shader(pixel_binary, gfx.device.clone()).unwrap();

    let mat = Arc::new(Material::new(&vertex_mod, &pixel_mod).unwrap());

    let mesh_data = Arc::new(MeshData::new(TRIANGLE_VERTICES.to_vec(), INDICES.to_vec()));
    create_model(mesh_data.clone(), mat.clone())
}
