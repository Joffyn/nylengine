use crate::pipeline::prelude::*;


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

