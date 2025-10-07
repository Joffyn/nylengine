use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::image::{ImageUsage, Image};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use vulkano::device::{Device};
use vulkano::device::physical::PhysicalDevice;
use winit::window::Window;

//#[allow(unused)]
//fn filter_devices(instance: Arc<Instance>, surface: Arc<Surface>) -> (Arc<PhysicalDevice>, u32)
//{
//
//    let device_extensions = DeviceExtensions 
//        {
//        khr_swapchain: true,
//        ..DeviceExtensions::empty()
//    };
//    instance
//        .enumerate_physical_devices()
//        .expect("Could not enumerate devices")
//        .filter(|p| p.supported_extensions().contains(&device_extensions))
//        .filter_map(|p| 
//            {
//                p.queue_family_properties()
//                    .iter()
//                    .enumerate()
//                    .position(|(i, q)|
//                        {
//                            q.queue_flags.contains(QueueFlags::GRAPHICS) 
//                                && p.surface_support(i as u32, &surface).unwrap_or(false)
//                        })
//                    .map(|q| (p, q as u32))
//            })
//        .min_by_key(|p, _| match p.properties().device_type 
//            {
//                PhysicalDeviceType::DiscreteGpu => 0,
//                PhysicalDeviceType::IntegratedGpu => 1,
//                PhysicalDeviceType::VirtualGpu => 2,
//                PhysicalDeviceType::Cpu => 3,
//                _ => 4,
//            })
//        .expect("No device is available")
//}

#[allow(unused)]
pub fn create_swapchain(phys_device: Arc<PhysicalDevice>, device: Arc<Device>, window: Arc<Window>, surface: Arc<Surface>)
-> (Arc<Swapchain>, Vec<Arc<Image>>)
{
    let caps = phys_device
        .surface_capabilities(&surface, Default::default())
        .expect("Failed to get surface capabilities");

    let dimensions = window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = phys_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;

    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo
        {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .expect("Failed to create swapchain")
}
