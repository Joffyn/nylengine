use std::ops::Deref;
use std::sync::Arc;
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::image::{Image, ImageLayout};
use vulkano::image::view::ImageView;
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::swapchain::{acquire_next_image, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::{sync, Validated, VulkanError, VulkanLibrary};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderingAttachmentInfo, RenderingInfo};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::render_pass::{AttachmentLoadOp, AttachmentStoreOp, Framebuffer};
use vulkano::sync::GpuFuture;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::pipeline::gfxpipeline::Material;
use crate::pipeline::instance::{create_instance, get_default_physical_device, get_device, get_queue_family_index};
use crate::pipeline::model::{create_triangle, Model};
use crate::pipeline::vertex::TestVertex;

pub struct RenderContext
{
    pub window: Arc<Window>,
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Swapchain>,
    pub attachment_image_views: Vec<Arc<ImageView>>,
    pub viewport: Viewport,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}
impl RenderContext
{
    fn new(app: &App, event_loop: &ActiveEventLoop) -> RenderContext
    {
        // The objective of this example is to draw a triangle on a window. To do so, we first need
        // to create the window. We use the `WindowBuilder` from the `winit` crate to do that here.
        //
        // Before we can render to a window, we must first create a `vulkano::swapchain::Surface`
        // object from it, which represents the drawable surface of a window. For that we must wrap
        // the `winit::window::Window` in an `Arc`.
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let surface = Surface::from_window(app.instance.clone(), window.clone()).unwrap();
        let window_size = window.inner_size();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        // Before we can draw on the surface, we have to create what is called a swapchain.
        // Creating a swapchain allocates the color buffers that will contain the image that will
        // ultimately be visible on the screen. These images are returned alongside the swapchain.
        let (swapchain, images) = renderer::pipeline::swapchain::create_swapchain(
            app.device.physical_device().clone(),
            app.device.clone(),
            window.clone(),
            surface.clone());
        // When creating the swapchain, we only created plain images. To use them as an attachment
        // for rendering, we must wrap then in an image view.
        //
        // Since we need to draw to multiple images, we are going to create a different image view
        // for each image.
        let attachment_image_views = window_size_dependent_setup(&images);

        // Dynamic viewports allow us to recreate just the viewport when the window is resized.
        // Otherwise we would have to recreate the whole pipeline.
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: window_size.into(),
            depth_range: 0.0..=1.0,
        };

        // In some situations, the swapchain will become invalid by itself. This includes for
        // example when the window is resized (as the images of the swapchain will no longer match
        // the window's) or, on Android, when the application went to the background and goes back
        // to the foreground.
        //
        // In this situation, acquiring a swapchain image or presenting it will return an error.
        // Rendering to an image of that swapchain will not produce any error, but may or may not
        // work. To continue rendering, we need to recreate the swapchain by creating a new
        // swapchain. Here, we remember that we need to do this for the next loop iteration.
        let recreate_swapchain = false;

        // In the loop below we are going to submit commands to the GPU. Submitting a command
        // produces an object that implements the `GpuFuture` trait, which holds the resources for
        // as long as they are in use by the GPU.
        //
        // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to
        // avoid that, we store the submission of the previous frame here.
        let previous_frame_end = Some(sync::now(app.device.clone()).boxed());

        RenderContext
        {
            window,
            surface,
            swapchain,
            attachment_image_views,
            viewport,
            recreate_swapchain,
            previous_frame_end,
        }
    }
}
pub struct App
{
    pub instance: Arc<Instance>,
    pub device: Arc<Device>,
    pub gfx_queue: Arc<Queue>,
    pub cmd_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub malloc: Arc<StandardMemoryAllocator>,
    pub rcx: Option<RenderContext>,
    triangle: Option<Model<TestVertex>>,

}
impl App
{
    fn new(event_loop: &EventLoop<()>) -> Self
    {
        let lib = VulkanLibrary::new().unwrap();
        let instance = create_instance(lib).unwrap();
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (phys_device, qfp) = get_default_physical_device(&device_extensions, instance.clone(), &event_loop);
        let qfi = get_queue_family_index(phys_device.clone()).unwrap_or(4) as u32;
        let (device, mut gfx_queue) = get_device(&device_extensions, phys_device.clone(), qfi).unwrap();
        let mut gfx_queue = gfx_queue.next().unwrap();

        let malloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let cmd_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));
        App
        {
            instance,
            device,
            gfx_queue,
            cmd_buffer_allocator,
            malloc,
            rcx: None,
            triangle: None
        }

    }

    fn begin_rendering_main_pass(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, image_index: u32)
    {
        builder
            .begin_rendering( RenderingInfo
            {
                color_attachments: vec![Some(RenderingAttachmentInfo
                {
                    image_view: Self.rcx.unwrap().attachment_image_views.clone()[image_index],
                    image_layout: ImageLayout::ColorAttachmentOptimal,
                    resolve_info: None,
                    load_op: AttachmentLoadOp::Clear,
                    store_op: AttachmentStoreOp::Store,
                    clear_value: Some([0.0, 0.0, 1.0, 1.0].into()),
                    _ne: vulkano::NonExhaustive(()),
                })],
                ..RenderingInfo::default()
            })
            .unwrap();
    }
    fn draw_triangle(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
    {
        builder.set_viewport(0, [Self.rcx.unwrap().viewport.clone()].into_iter().collect()
            .unwrap()
            .bind_pipeline_graphics(self.triangle.unwrap().material.gfxpipeline)
            .unwrap()
            .bind_vertex_buffers(0, self.triangle.unwrap().vertex_buffer))
            .unwrap();

        unsafe { builder.draw(
            self.triangle.unwrap().vertex_buffer.deref().len() as u32,
            1,
            0,
            0)};
    }
    fn end_main_pass(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, acquire_future: SwapchainAcquireFuture, image_index: u32)
    {
        builder
            .end_rendering()
            .unwrap();

        let command_buffer = builder.build().unwrap();
        let future = self.rcx
            .unwrap()
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.gfx_queue.clone(), command_buffer)
            .unwrap()
            // The color output is now expected to contain our triangle. But in order to
            // show it on the screen, we have to *present* the image by calling
            // `then_swapchain_present`.
            //
            // This function does not actually present the image immediately. Instead it
            // submits a present command at the end of the queue. This means that it will
            // only be presented once the GPU has finished executing the command buffer
            // that draws the triangle.
            .then_swapchain_present(
                self.gfx_queue.clone(),
                SwapchainPresentInfo
                {
                    swapchain: self.rcx.unwrap().swapchain,
                    image_index,
                    present_id: None,
                    present_mode: None,
                    present_region: vec![],
                    _ne: vulkano::NonExhaustive(()),
                },
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                self.rcx.unwrap().previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.rcx.unwrap().recreate_swapchain = true;
                self.rcx.unwrap().previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                self.rcx.unwrap().previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
}

impl ApplicationHandler for App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        self.rcx = Some(RenderContext::new(&self, event_loop));
        self.triangle = Some(create_triangle(&self));
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop)
    {
        let rcx = self.rcx.as_mut().unwrap();
        rcx.window.request_redraw();
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent)
    {
        match event
        {
            WindowEvent::Resized(_) =>
                {
                    self.rcx.unwrap().recreate_swapchain = true;
                }
            WindowEvent::CloseRequested => { println!("Window closing!"); event_loop.exit(); },
            WindowEvent::RedrawRequested =>
                {
                    let window_size = self.rcx.unwrap().window.inner_size();

                    // Do not draw the frame when the screen size is zero. On Windows, this can occur
                    // when minimizing the application.
                    if window_size.width == 0 || window_size.height == 0 {
                        return;
                    }

                    let mut rcx = self.rcx.unwrap();
                    // It is important to call this function from time to time, otherwise resources
                    // will keep accumulating and you will eventually reach an out of memory error.
                    // Calling this function polls various fences in order to determine what the GPU
                    // has already processed, and frees the resources that are no longer needed.
                    rcx.previous_frame_end.as_mut().unwrap().cleanup_finished();
                    let (image_index, acquire_future) = match check_swapchain(&mut rcx, &window_size)
                    {
                        Ok((i, a)) => (i, a),
                        Err(e) => return,
                    };
                    let mut builder = AutoCommandBufferBuilder::primary(
                        self.cmd_buffer_allocator.clone(),
                        self.gfx_queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    ).unwrap();
                    self.begin_rendering_main_pass(&mut builder, image_index);
                    self.draw_triangle(&mut builder);
                    self.end_main_pass(&mut builder, acquire_future, image_index);
                },
            _ => (),
        }
    }
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(images: &[Arc<Image>]) -> Vec<Arc<ImageView>>
{
    images
        .iter()
        .map(|image| ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()
}
fn check_swapchain(rcx: &mut RenderContext, window_size: &PhysicalSize<u32>) -> Option<(u32, SwapchainAcquireFuture)>
{
    // Whenever the window resizes we need to recreate everything dependent on the
    // window size. In this example that includes the swapchain, the framebuffers and
    // the dynamic state viewport.
    if rcx.recreate_swapchain
    {
        let (new_swapchain, new_images) = rcx
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: window_size.into(),
                ..rcx.swapchain.create_info()
            })
            .expect("failed to recreate swapchain");

        rcx.swapchain = new_swapchain;

        // Now that we have new swapchain images, we must create new image views from
        // them as well.
        rcx.attachment_image_views = window_size_dependent_setup(&new_images);

        rcx.viewport.extent = window_size.into();

        rcx.recreate_swapchain = false;
    }

    // Before we can draw on the output, we have to *acquire* an image from the
    // swapchain. If no image is available (which happens if you submit draw commands
    // too quickly), then the function will block. This operation returns the index of
    // the image that we are allowed to draw upon.
    //
    // This function can block if no image is available. The parameter is an optional
    // timeout after which the function call will return an error.

    let (image_index, suboptimal, acquire_future) = match acquire_next_image(
        rcx.swapchain.clone(),
        None,
    )
        .map_err(Validated::unwrap)
    {
        Ok(r) => r,
        Err(VulkanError::OutOfDate) => {
            rcx.recreate_swapchain = true;
            return None;
        }
        Err(e) => panic!("failed to acquire next image: {e}"),
    };

    // `acquire_next_image` can be successful, but suboptimal. This means that the
    // swapchain image will still work, but it may not display correctly. With some
    // drivers this can be when the window resizes, but it may not cause the swapchain
    // to become out of date.
    if suboptimal
    {
        rcx.recreate_swapchain = true;
    }
    Some((image_index, acquire_future))
}