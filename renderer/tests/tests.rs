pub mod common;

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use vulkano::memory::allocator::StandardMemoryAllocator;
    use vulkano::device::DeviceExtensions;
    use vulkano::VulkanLibrary;
    use renderer::pipeline::buffers::{self, create_vertex_buffer};
    use renderer::pipeline::instance::{create_instance, get_device, get_queue_family_index};
    use renderer::pipeline::vertex::TestVertex;
    use renderer::pipeline::renderpass::{self, create_renderpass};
    use renderer::pipeline::image::{self, create_image};
    use renderer::pipeline::window::{self, create_default_window};
    use renderer::pipeline::surface::{self, create_surface};
    use renderer::pipeline::instance;
    use renderer::pipeline::swapchain::{self, create_swapchain};
    use crate::common::{get_test_physical_device, TEST_DEVICE_CONTEXT};
    

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
        let (device, queue) = &*TEST_DEVICE_CONTEXT;
    }
    #[test]
    fn vertex_buffer_test()
    {
        let (device, queue) = &*TEST_DEVICE_CONTEXT;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
        for vert in &mut verts { 
            vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];  
        }
        let vbuffer = create_vertex_buffer(memory_allocator.clone(), verts.to_vec());
    }
    #[test]
    fn renderpass_test()
    {
        let (device, queue) = &*TEST_DEVICE_CONTEXT;
        let pass = create_renderpass(device.clone()).expect("Renderpass failed to be created");
    }
    #[test]
    fn create_image_test()
    {
        let (device, queue) = &*TEST_DEVICE_CONTEXT;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let image = create_image(memory_allocator).expect("Image failed to be created");
    }

    #[test]
    fn window_test()
    {
        let (device, queue) = &*TEST_DEVICE_CONTEXT;
        let window = create_default_window();
        let surface = create_surface(window.clone(), device.instance().clone());
    }
    #[test]
    fn swapchain_test()
    {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let lib = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let instance = create_instance(lib).unwrap();
        let phys_device = get_test_physical_device(instance.clone()).unwrap();
        let qfi = get_queue_family_index(phys_device.clone()).expect("No queue family supporting graphics was found") as u32;
        let (device, _) = get_device(&device_extensions, phys_device.clone(), qfi).unwrap();
        let window = create_default_window();
        let surface = create_surface(window.clone(), instance.clone());
        let (swapchain, _) = create_swapchain(phys_device, device, window, surface);
    }

}

