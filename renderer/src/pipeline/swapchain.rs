//use std::sync::Arc;
//use winit::window::Window;
//
//#[allow(unused)]
//pub fn create_swapchain(phys_device: Arc<PhysicalDevice>, device: Arc<Device>, window: Arc<Window>, surface: Arc<Surface>)
//-> (Arc<Swapchain>, Vec<Arc<Image>>)
//{
//    let caps = phys_device
//        .surface_capabilities(&surface, Default::default())
//        .expect("Failed to get surface capabilities");
//
//    let dimensions = window.inner_size();
//    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
//    let image_format = phys_device
//        .surface_formats(&surface, Default::default())
//        .unwrap()[0]
//        .0;
//
//    Swapchain::new(
//        device.clone(),
//        surface.clone(),
//        SwapchainCreateInfo
//        {
//            min_image_count: caps.min_image_count + 1,
//            image_format,
//            image_extent: dimensions.into(),
//            image_usage: ImageUsage::COLOR_ATTACHMENT,
//            composite_alpha,
//            ..Default::default()
//        },
//    )
//    .expect("Failed to create swapchain")
//}
