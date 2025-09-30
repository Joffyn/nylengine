use vulkano::VulkanLibrary;
pub mod pipeline;

use crate::pipeline::instance::{self, create_instance};

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");

    create_instance(library);
    
}
