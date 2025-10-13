use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::VulkanError;
use vulkano::Validated;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::swapchain::Surface;
use std::sync::Arc;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub fn create_instance(lib: Arc<VulkanLibrary>) -> Result<Arc<Instance>, Validated<VulkanError>>
{
    Instance::new(
        lib,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )
}
#[allow(unused)]
pub fn get_default_physical_device(device_extensions: &DeviceExtensions, instance: Arc<Instance>, surface: Arc<Surface>)
-> (Arc<PhysicalDevice>, u32)
{
    instance.clone()
        .enumerate_physical_devices()
        .expect("Could not enumerate devices")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| 
            {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)|
                        {
                            q.queue_flags.contains(QueueFlags::GRAPHICS) 
                                && p.surface_support(i as u32, &surface.clone()).unwrap_or(false)
                        })
                    .map(|q| (p, q as u32))
            })
        .min_by_key(|(p, _)| match p.properties().device_type 
            {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            })
        .expect("No device is available")
}

#[allow(dead_code)]
pub fn get_queue_family_index(phys_device: Arc<PhysicalDevice>) -> Option<usize>
{
    phys_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_qfi, qfp)| { qfp.queue_flags.contains(QueueFlags::GRAPHICS) })
}

#[allow(dead_code)]
pub fn get_device(device_extensions: &DeviceExtensions, phys_device: Arc<PhysicalDevice>, qfi: u32)
-> Result<(Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>), Validated<VulkanError>>
{
    Device::new(
        phys_device.clone(), 
        DeviceCreateInfo 
        { 
            queue_create_infos: vec![QueueCreateInfo 
                {
                    queue_family_index: qfi, ..Default::default()
                }], 
            enabled_extensions: *device_extensions , ..Default::default()
        },
    )
}
