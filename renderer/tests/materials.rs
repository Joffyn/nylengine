pub mod common;

#[cfg(test)]
mod tests {
//    use std::path::Path;
//    use std::sync::Arc;
//    use vulkano::memory::allocator::StandardMemoryAllocator;
//    use renderer::pipeline::buffers::{self, create_vertex_buffer};
//    use renderer::pipeline::instance::{create_instance, get_device, get_queue_family_index};
//    use renderer::pipeline::vertex::TestVertex;
//    use renderer::pipeline::renderpass::{self, create_renderpass};
//    use renderer::pipeline::image::{self, create_image};
//    use renderer::pipeline::window::{self, create_default_window};
//    use renderer::pipeline::surface::{self, create_surface};
//    use renderer::pipeline::instance;
//    use renderer::pipeline::swapchain::{self, create_swapchain};
//    use renderer::pipeline::shaderloading::{load_shader, compile_shader};
//    use renderer::pipeline::gfxpipeline::*;
//    use crate::common::{get_test_physical_device, TEST_DEVICE_CONTEXT};
//    use shaderc::ShaderKind;
//
//    #[test]
//    fn create_test_material()
//    {
//        let (device, _) = &*TEST_DEVICE_CONTEXT;
//        let vertpath = Path::new("shaders/testshader.vs");
//        let vertspirv = compile_shader(ShaderKind::Vertex, vertpath).expect("Couldn't compile shader");
//        let vertex = load_shader(vertspirv, device.clone()).expect("Couldn't load shader");
//
//        let fragpath = Path::new("shaders/testshader.ps");
//        let fragspirv = compile_shader(ShaderKind::Fragment, fragpath).expect("Couldn't compile shader");
//        let frag = load_shader(fragspirv, device.clone()).expect("Couldn't load shader");
//
//        let renderpass = create_renderpass(device.clone()).expect("Couldn't create renderpass");
//        let mat = Material::new::<TestVertex>(
//            &vertex,
//            &frag,
//            device.clone(),
//            renderpass).expect("Couldn't create material");
//    }
}
