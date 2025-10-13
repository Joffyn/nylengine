pub mod instance;
pub mod buffers;
pub mod vertex;
pub mod renderpass;
pub mod image;
pub mod window;
pub mod surface;
pub mod swapchain;
pub mod gfxpipeline;
pub mod shaderloading;
pub mod commandbuffer;
pub mod framebuffer;
pub mod prelude;



//#[cfg(test)]
//mod tests {
//    use std::sync::Arc;
//    use vulkano::memory::allocator::StandardMemoryAllocator;
//    use crate::pipeline::buffers::{self, create_vertex_buffer};
//    use crate::pipeline::instance::{create_instance, get_device, get_queue_family_index};
//    //use crate::pipeline::instance::DEVICE_CONTEXT;
//    use crate::pipeline::vertex::TestVertex;
//    use crate::pipeline::renderpass::{self, create_render_pass};
//    use crate::pipeline::image::{self, create_image};
//    use crate::pipeline::window::{self, create_default_window};
//    use crate::pipeline::surface::{self, create_surface};
//    
//    #[allow(dead_code)]
//    fn get_test_physical_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>>
//    {
//        instance.enumerate_physical_devices()
//        .expect("Couldnt enumerate physical devices")
//        .next()
//    }
//    
//    //Maybe handle errors? Probably not neccessary in this case
//    #[allow(dead_code)]
//    fn create_test_device_and_queue() -> (Arc<Device>, Arc<Queue>)
//    {
//        let device_extensions = DeviceExtensions {
//            khr_swapchain: true,
//            ..DeviceExtensions::empty()
//        };
//
//        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
//        let instance = create_instance(library).expect("No instance could be created");
//        let phys_device = get_test_physical_device(instance.clone()).expect("No physical graphics device was found");
//        let qfi = get_queue_family_index(phys_device.clone()).expect("No queue family supporting graphics was found") as u32;
//        let mut device_queue = get_device(&device_extensions, phys_device.clone(), qfi).expect("Device could not be created");
//        (device_queue.0, device_queue.1.next().expect("Device queue was empty"))
//    }
//
//    #[allow(dead_code)]
//    static TEST_DEVICE_CONTEXT: Lazy<(Arc<Device>, Arc<Queue>)> = Lazy::new(|| {
//        use crate::pipeline::instance::create_test_device_and_queue;
//        create_test_device_and_queue()
//    });
//
//    #[test]
//    fn vertex_test() 
//    {
//        let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
//        for vert in &mut verts { 
//            vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];  
//        }
//        for vert in &verts { println!("Vertex: {:?}", vert); }
//        assert_eq!(verts.len(), 3);
//        assert_ne!(verts.first().expect("Vertex somehow not existing").pos, verts.last().expect("Vertex somehow not existing").pos);
//    }
//
//    #[test]
//    fn init_test()
//    {
//        use crate::pipeline::instance;
//        let (device, queue) = &*TEST_DEVICE_CONTEXT;
//    }
//    #[test]
//    fn vertex_buffer_test()
//    {
//        let (device, queue) = &*TEST_DEVICE_CONTEXT;
//        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
//        let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
//        for vert in &mut verts { 
//            vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];  
//        }
//        let vbuffer = create_vertex_buffer(memory_allocator.clone(), verts.to_vec());
//    }
//    #[test]
//    fn renderpass_test()
//    {
//        let (device, queue) = &*TEST_DEVICE_CONTEXT;
//        let pass = create_render_pass(device.clone()).expect("Renderpass failed to be created");
//    }
//    #[test]
//    fn create_image_test()
//    {
//        let (device, queue) = &*TEST_DEVICE_CONTEXT;
//        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
//        let image = create_image(memory_allocator).expect("Image failed to be created");
//    }
//
//    #[test]
//    fn window_test()
//    {
//        let (device, queue) = &*TEST_DEVICE_CONTEXT;
//        let window = create_default_window();
//        let surface = create_surface(window.clone(), device.instance().clone());
//
//    }
//
//}
