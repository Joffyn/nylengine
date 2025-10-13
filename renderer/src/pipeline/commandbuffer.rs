use crate::pipeline::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum RenderError 
{
    #[error("Command buffer execution failed: {0}")]
    CommandExec(#[from] vulkano::command_buffer::CommandBufferExecError),
    #[error("Error with commandbuffer: {0}")]
    VulkanError(#[from] vulkano::Validated<vulkano::VulkanError>)
}
pub fn begin_commandbuffer_recording(queue: Arc<Queue>, device: Arc<Device>)
-> Result<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, Validated<VulkanError>>
{

    let cba = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    AutoCommandBufferBuilder::primary(
        &cba,
        queue.clone().queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
}
pub fn end_commandbuffer_recording(cmdbuffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>)
-> Result<Arc<PrimaryAutoCommandBuffer>, Validated<VulkanError>>
{
    cmdbuffer_builder.build()
}

pub fn execute_commandbuffer(
    device: Arc<Device>,
    queue: Arc<Queue>,
    cmd_buffer: Arc<PrimaryAutoCommandBuffer>)
    -> Result<(), RenderError>
{
    let future = sync::now(device.clone());
    let future = future.then_execute(queue.clone(), cmd_buffer)?;
    let future = future.then_signal_fence_and_flush()?;

    future.wait(None)?;
    Ok(())
}

