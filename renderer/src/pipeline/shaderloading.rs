use vulkano::shader::{ShaderModule, ShaderModuleCreateInfo};
use vulkano::device::Device;
use vulkano::{Validated, VulkanError};
use std::fmt::Display;
use std::ops::Add;
use shaderc::Compiler;
use shaderc::ShaderKind;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::path::Path;
use std::process::Command;

static COMPILER_CONTEXT: Lazy<Compiler> = Lazy::new(|| {
    shaderc::Compiler::new().unwrap()
});


pub fn compile_shader(shaderkind: ShaderKind, path: &Path) -> Result<(), Box<dyn std::error::Error>>
{
    let output_path= path
        .to_str()
        .unwrap()
        .to_string()
        .strip_suffix(".hlsl")
        .unwrap()
        .to_string() + ".spv";

    let shader = match shaderkind
    {
        ShaderKind::Vertex => "vs_6_0",
        ShaderKind::Fragment => "ps_6_0",
        _ => todo!(),
    };
    let status = Command::new("dxc")
        .args([
            "-T", shader,      // target: vertex shader (adjust per file)
            "-E", "main",        // entry point
            "-spirv",            // output SPIR-V
            "-Fo", output_path.as_str(),
            path.to_str().unwrap(),
        ])
        .status()?;

    if status.success() {
        println!("✅ Compiled");
    } else {
        eprintln!("Couldn't compile");
    }
    Ok(())
}
pub fn load_shader(spirv: Vec<u32>, device: Arc<Device>) -> Result<Arc<ShaderModule>, Validated<VulkanError>>
{
    let create_info = ShaderModuleCreateInfo::new(spirv.as_slice());
    unsafe { ShaderModule::new(device.clone(), create_info) }
}
