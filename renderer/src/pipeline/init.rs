use crate::pipeline::{
    buffers::create_vertex_buffer, 
    //commandbuffer::{begin_commandbuffer_recording, end_commandbuffer_recording, execute_commandbuffer}, 
    framebuffer::{self, get_framebuffers}, 
    instance::{self, get_queue_family_index, get_device, create_instance, get_default_physical_device}, 
    prelude::*, 
    renderpass::{self, basic_draw_call, begin_renderpass, create_renderpass, end_renderpass}, 
    surface::{self, create_surface}, 
    vertex::*,
    shaderloading::*,
    gfxpipeline::*,
    swapchain::{self, create_swapchain}, window::create_default_window};

//pub fn init_renderer_basic_test() -> Result<(), Box<dyn std::error::Error>>
//{
//    let library = VulkanLibrary::new()?;
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
//    let vtxbuffer = create_vertex_buffer(malloc.clone(), verts);
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
//    let builder = begin_commandbuffer_recording(queue.clone(), device.clone())?;
//    begin_renderpass(&mut builder, framebuffer.clone(), mat.gfxpipeline.clone());
//    basic_draw_call(&mut builder, vertex_buffer, &mat);
//    end_renderpass(&mut builder);
//    let cmdbuffer = end_commandbuffer_recording(builder)?;
//
//    execute_commandbuffer(device.clone(), queue.clone(), cmdbuffer)?;
//
//    
//
//    //(device_queue.0, device_queue.1.next().expect("Device queue was empty"));
//
//    Ok(())
//}

