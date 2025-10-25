pub mod common;

//#[cfg(test)]
//mod tests {
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
//    use crate::common::{get_test_physical_device, TEST_DEVICE_CONTEXT};
//    use shaderc::ShaderKind;
//
//    #[test]
//    fn load_and_compile_shader_test()
//    {
//        let (device, _) = &*TEST_DEVICE_CONTEXT;
//        let path = Path::new("shaders/testshader.vs");
//        let spirv = compile_shader(ShaderKind::Vertex, path).expect("Couldn't compile shader");
//        let shader = load_shader(spirv, device.clone()).expect("Couldn't load shader");
//    }
//}
