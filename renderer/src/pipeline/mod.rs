pub mod instance;
pub mod buffers;
pub mod vertex;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use vulkano::memory::allocator::StandardMemoryAllocator;
    use crate::pipeline::buffers::{self, create_vertex_buffer};
    use crate::pipeline::instance::DEVICE_CONTEXT;
    use crate::pipeline::vertex::TestVertex;

    #[test]
    fn vertex_test() 
    {
        let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
        for vert in &mut verts { 
            vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];  
        }
        for vert in &verts { println!("Vertex: {:?}", vert); }
        assert_eq!(verts.len(), 3);
        assert_ne!(verts.first().expect("Vertex somehow not existing").pos, verts.last().expect("Vertex somehow not existing").pos);
    }

    #[test]
    fn init_test()
    {
        use crate::pipeline::instance;
        let (device, queue) = &*DEVICE_CONTEXT;
    }
    #[test]
    fn vertex_buffer_test()
    {
        let (device, queue) = &*DEVICE_CONTEXT;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
        for vert in &mut verts { 
            vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];  
        }
        let vbuffer = create_vertex_buffer(memory_allocator.clone(), verts.to_vec());
    }

}
