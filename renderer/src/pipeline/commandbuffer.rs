use crate::pipeline::{prelude::*, gfxpipeline::*, renderpass::{basic_draw_call, begin_renderpass, end_renderpass}};


pub fn record_multiple_commandbuffers<T>(
    queue: Arc<Queue>,
    device: Arc<Device>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    mat: &Material,
    vertex_buffer: &Subbuffer<[T]>)
-> Result<Vec<Arc<PrimaryAutoCommandBuffer>>, RendererError>
    where T : Vertex,
{
    let cba = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );
    Ok(framebuffers
        .iter()
        .map(|framebuffer|
            {
                let mut builder = AutoCommandBufferBuilder::primary(
                    &cba,
                    queue.clone().queue_family_index(),
                    CommandBufferUsage::MultipleSubmit,
                ).unwrap();
                begin_renderpass(&mut builder, &framebuffer.clone(), mat.gfxpipeline.clone());
                basic_draw_call::<T>(&mut builder, vertex_buffer, mat).unwrap();
                end_renderpass(&mut builder);
                builder.build().unwrap()
            })
        .collect())
}

pub fn begin_commandbuffer_recording(queue: Arc<Queue>, device: Arc<Device>)
-> Result<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, RendererError>
{

    let cba = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    Ok(AutoCommandBufferBuilder::primary(
        &cba,
        queue.clone().queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )?)
}
pub fn end_commandbuffer_recording(cmdbuffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
-> Result<Arc<PrimaryAutoCommandBuffer>, RendererError>
{
    Ok(cmdbuffer_builder.build()?)
}

pub fn execute_commandbuffer(
    device: Arc<Device>,
    queue: Arc<Queue>,
    cmd_buffer: Arc<PrimaryAutoCommandBuffer>)
    -> Result<(), RendererError>
{
    let future = sync::now(device.clone());
    let future = future.then_execute(queue.clone(), cmd_buffer)?;
    let future = future.then_signal_fence_and_flush()?;

    future.wait(None)?;
    Ok(())
}

