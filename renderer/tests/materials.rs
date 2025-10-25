pub mod common;

//#[cfg(test)]
//mod tests {
//    use std::path::Path;
//    use renderer::pipeline::vertex::TestVertex;
//    use renderer::pipeline::shaderloading::{load_shader, compile_shader};
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
//        let mat = Material::new::<TestVertex>(
//            &vertex,
//            &frag,
//            device.clone(),
//            ).expect("Couldn't create material");
//    }
//}
