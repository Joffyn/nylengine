use vulkano::shader::{ShaderModule, ShaderModuleCreateInfo};
use vulkano::device::Device;
use vulkano::{Validated, VulkanError};
use std::fmt::Display;
use shaderc::Compiler;
use shaderc::ShaderKind;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::path::Path;

static COMPILER_CONTEXT: Lazy<Compiler> = Lazy::new(|| {
    shaderc::Compiler::new().unwrap()
});


//#[derive(Debug, Display)]
//pub enum ShaderType
//{
//    Vertex,
//    Fragment,
//}

pub fn compile_shader(shaderkind: ShaderKind, path: &Path) -> Result<Vec<u32>, Box<dyn std::error::Error>>
{
    let contents = std::fs::read_to_string(path)?;
    let compiler = &*COMPILER_CONTEXT;

    let spirv = compiler.compile_into_spirv(&contents, shaderkind, path.to_str().unwrap(), "main",None)?;

    Ok(spirv.as_binary().to_vec())
}
pub fn load_shader(spirv: Vec<u32>, device: Arc<Device>) -> Result<Arc<ShaderModule>, Validated<VulkanError>>
{
    let createinfo = ShaderModuleCreateInfo::new(spirv.as_slice());
    unsafe { ShaderModule::new(device.clone(), createinfo) }
}
