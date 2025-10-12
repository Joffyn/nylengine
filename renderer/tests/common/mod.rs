use once_cell::sync::Lazy;
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::Instance;
use vulkano::VulkanLibrary;
use std::sync::Arc;
use renderer::pipeline::instance::{get_queue_family_index, create_instance, get_device};

pub fn get_test_physical_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>>
{
    instance.enumerate_physical_devices()
        .expect("Couldnt enumerate physical devices")
        .next()
}
fn create_test_device_and_queue() -> (Arc<Device>, Arc<Queue>)
{
    let device_extensions = DeviceExtensions {
        khr_swapchain: false,
        ..DeviceExtensions::empty()
    };

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = create_instance(library).expect("No instance could be created");
    let phys_device = get_test_physical_device(instance.clone()).expect("No physical graphics device was found");
    let qfi = get_queue_family_index(phys_device.clone()).expect("No queue family supporting graphics was found") as u32;
    let mut device_queue = get_device(&device_extensions, phys_device.clone(), qfi).expect("Device could not be created");
    (device_queue.0, device_queue.1.next().expect("Device queue was empty"))
}

pub static TEST_DEVICE_CONTEXT: Lazy<(Arc<Device>, Arc<Queue>)> = Lazy::new(|| {
    create_test_device_and_queue()
});
