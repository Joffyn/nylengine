use std::sync::Arc;
use wgpu::{include_spirv, Device, SurfaceConfiguration};
use crate::pipeline::material::{Material, MaterialInstance};
use crate::pipeline::model::Model;
use crate::pipeline::vertex::TestVertex;
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

pub fn create_triangle(device: &Device, config: &SurfaceConfiguration) -> Model
{
    let vertex_shader = device.create_shader_module(include_spirv!("shaders/vertex.spv"));
    let pixel_shader = device.create_shader_module(include_spirv!("shaders/pixel.spv"));

    let mat = Material::new::<TestVertex>(&vertex_shader, &pixel_shader, &device, &config);
    let mat_inst = Arc::new(MaterialInstance {base_material: Arc::new(mat)});

    Model::new(&device, mat_inst.clone(), TRIANGLE_VERTICES, INDICES)
}