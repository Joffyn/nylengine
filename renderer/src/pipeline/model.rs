use core::slice::SlicePattern;
use std::iter::TrustedRandomAccessNoCoerce;
use std::path::Path;
use std::sync::Arc;
use once_cell::sync::Lazy;
use shaderc::{Compiler, ShaderKind};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::device::Device;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryAllocatePreference, MemoryTypeFilter};
use vulkano::NonExhaustive;
use vulkano::sync::Sharing;
use winit::window::CursorIcon::Default;
use renderer::pipeline::shaderloading::{compile_shader, load_shader};
use crate::pipeline::app::App;
//use bytemuck::Pod;
use crate::pipeline::material::{Material};
use crate::pipeline::vertex::{IndexBuffer, TestVertex, Vertex, VertexBuffer};

pub struct Model<T>
where T : Vertex
{
    pub material: Arc<Material>,
    pub vertex_buffer: VertexBuffer<T>,
    pub index_buffer: IndexBuffer,
}

impl<T> Model<T>
{
    pub fn new<T>(app: &App, material: Arc<Material>, verts: &[T], indices: &[u16]) -> Model<T>
    where T: Vertex
    {
        Model
        {
            material: material.clone(),
            vertex_buffer: VertexBuffer::new(app.malloc.clone(), &verts),
            index_buffer: IndexBuffer::new(app.malloc.clone(), &indices),
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
pub fn create_triangle<T>(app: &App) -> Model<T>
{
    let vertex_shader = compile_shader(ShaderKind::Vertex, Path::new("shaders/vertex.hlsl")).unwrap();
    let pixel_shader = compile_shader(ShaderKind::Fragment, Path::new("shaders/pixel.hlsl")).unwrap();

    let vertex_mod = load_shader(vertex_shader, app.device.clone()).unwrap();
    let pixel_mod = load_shader(pixel_shader, app.device.clone()).unwrap();

    let mat = Arc::new(Material::new::<TestVertex>(&vertex_mod, &pixel_mod, &app, ).unwrap());

    Model::new(&app, mat.clone(), TRIANGLE_VERTICES, INDICES)
}


