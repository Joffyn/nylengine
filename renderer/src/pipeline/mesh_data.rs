use std::collections::HashMap;
use std::hash::Hash;
use once_cell::sync::Lazy;
use crate::pipeline::vertex::{IndexBuffer, TestVertex, VertexBuffer};
use std::sync::RwLock;

static MESH_DATA: Lazy<RwLock<HashMap<String, MeshData>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    RwLock::new(m)
});
pub struct MeshData
{
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}
//Add model/mesh loading
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
