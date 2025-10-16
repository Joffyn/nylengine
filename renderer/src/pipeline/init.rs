//use crate::pipeline::prelude::*;
use shaderc::*;

//pub fn init_renderer_basic_test() -> Result<(), Box<dyn std::error::Error>>
//{
//    let lib = VulkanLibrary::new()?;
//    let instance = create_instance(lib)?;
//    let window = create_default_window();
//    let device_extensions = DeviceExtensions {
//        khr_swapchain: true,
//        ..DeviceExtensions::empty()
//    };
//    let surface = create_surface(window.clone(), instance.clone());
//
//
//
//    let phys_device = get_default_physical_device(&device_extensions, instance.clone(), surface.clone())?;
//    let qfi = get_queue_family_index(phys_device.clone())? as u32;
//    let (device, mut queue) = get_device(&device_extensions, phys_device.clone(), qfi)?;
//    let mut queue = queue.next()?;
//    let (swapchain, images) = create_swapchain(phys_device.clone(), device.clone(), window.clone(), surface.clone());
//
//    let renderpass = create_renderpass(device.clone())?;
//    let framebuffers = get_framebuffers(&images.as_slice(), &renderpass);
//
//    let malloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
//
//    let mut verts = [TestVertex{ pos: [0f32,0f32]}; 3];
//    for vert in &mut verts
//    {
//        vert.pos = [rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)];
//    }
//
//    let vertex_buffer = create_vertex_buffer(malloc.clone(), verts);
//
//    let vertpath = Path::new("shaders/testshader.vs");
//    let vertspirv = compile_shader(ShaderKind::Vertex, vertpath)?;
//    let vertex = load_shader(vertspirv, device.clone())?;
//
//    let fragpath = Path::new("shaders/testshader.ps");
//    let fragspirv = compile_shader(ShaderKind::Fragment, fragpath)?;
//    let frag = load_shader(fragspirv, device.clone())?;
//
//    let mat = Material::new::<TestVertex>(
//        &vertex,
//        &frag,
//        device.clone(),
//        renderpass)?;
//
//
//    let cmdbuffers = record_multiple_commandbuffers(&queue, &device, &framebuffers, &mat, &vertex_buffer)?;
//    execute_commandbuffer(device.clone(), queue.clone(), cmdbuffer)?;
//
//    Ok(())
//}

