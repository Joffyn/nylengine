use std::sync::Arc;
use vulkano::instance::Instance;


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
fn create_swapchain(instance: Arc<Instance>) {
     instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");
}
