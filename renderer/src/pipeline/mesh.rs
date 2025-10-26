use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::sync::Arc;
use once_cell::sync::Lazy;
use shaderc::{Compiler, ShaderKind};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::Device;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryAllocatePreference, MemoryTypeFilter};
use vulkano::NonExhaustive;
//use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::sync::Sharing;
use winit::window::CursorIcon::Default;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use crate::pipeline::shaderloading::{compile_shader, load_shader};
use crate::pipeline::app::{App, GFX_CONTEXT};
//use bytemuck::Pod;
use crate::pipeline::material::{Material};
use crate::pipeline::vertex::{IndexBuffer, TestVertex, VertexBuffer};


use std::sync::RwLock;

static MESHES: Lazy<RwLock<HashMap<String, MeshData>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Cube".to_string(), create_triangle());
    RwLock::new(m)
});


pub struct MeshData
{
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}

impl MeshData
{
    pub fn new(verts: Vec<TestVertex>, indices: Vec<u16>) -> MeshData
    {
        MeshData
        {
            vertex_buffer: VertexBuffer::new(verts),
            index_buffer: IndexBuffer::new(indices),
        }
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
pub fn create_triangle() -> MeshData
{
    let vertex_shader = compile_shader(ShaderKind::Vertex, Path::new("shaders/vertex.hlsl")).unwrap();
    let pixel_shader = compile_shader(ShaderKind::Fragment, Path::new("shaders/pixel.hlsl")).unwrap();

    let gfx = GFX_CONTEXT.read().unwrap();
    let gfx = gfx.as_ref().unwrap();

    let vertex_mod = load_shader(vertex_shader, gfx.device.clone()).unwrap();
    let pixel_mod = load_shader(pixel_shader, gfx.device.clone()).unwrap();

    let mat = Arc::new(Material::new(&vertex_mod, &pixel_mod).unwrap());

    MeshData::new(TRIANGLE_VERTICES.to_vec(), INDICES.to_vec())
}


