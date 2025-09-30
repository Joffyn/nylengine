use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use std::sync::Arc;

pub fn create_instance(lib: Arc<VulkanLibrary>) -> Arc<Instance>
{
    Instance::new(
        lib,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    ).expect("Expected an instance to be created")
}


