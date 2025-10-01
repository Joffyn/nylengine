use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags, physical::PhysicalDevice};
use vulkano::VulkanError;
use vulkano::Validated;
use std::sync::Arc;
use once_cell::sync::Lazy;
#[allow(dead_code)]
fn create_instance(lib: Arc<VulkanLibrary>) -> Result<Arc<Instance>, Validated<VulkanError>>
{
    Instance::new(
        lib,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
}
#[allow(dead_code)]
fn get_physical_device(instance: Arc<Instance>) -> Option<Arc<PhysicalDevice>>
{
    instance.enumerate_physical_devices()
    .expect("Couldnt enumerate physical devices")
    .next()
}

#[allow(dead_code)]
fn get_queue_family_index(phys_device: Arc<PhysicalDevice>) -> Option<usize>
{
    phys_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_qfi, qfp)| { qfp.queue_flags.contains(QueueFlags::GRAPHICS) })
}

#[allow(dead_code)]
fn get_device(phys_device: Arc<PhysicalDevice>, qfi: u32) -> Result<
(Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>), Validated<VulkanError>>
{
    Device::new(
    phys_device, 
    DeviceCreateInfo { queue_create_infos: vec![QueueCreateInfo {queue_family_index: qfi, ..Default::default()}], ..Default::default()},
    )
}

//Maybe handle errors? Probably not neccessary in this case
#[allow(dead_code)]
fn create_device_and_queue() -> (Arc<Device>, Arc<Queue>)
{
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = create_instance(library).expect("No instance could be created");
    let phys_device = get_physical_device(instance.clone()).expect("No physical graphics device was found");
    let qfi = get_queue_family_index(phys_device.clone()).expect("No queue family supporting graphics was found") as u32;
    let mut device_queue = get_device(phys_device.clone(), qfi).expect("Device could not be created");
    (device_queue.0, device_queue.1.next().expect("Device queue was empty"))
}




#[allow(dead_code)]
pub(crate) static DEVICE_CONTEXT: Lazy<(Arc<Device>, Arc<Queue>)> = Lazy::new(|| {
    use crate::pipeline::instance::create_device_and_queue;
    create_device_and_queue()
});

