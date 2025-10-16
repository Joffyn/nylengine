use std::sync::Arc;

//pub fn create_vertex_buffer<T>(malloc: Arc<StandardMemoryAllocator>, verts: Vec<T>) -> Subbuffer<[T]>
//where T: Vertex,
//{
//    Buffer::from_iter(
//    malloc.clone(),
//    BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default()},
//    AllocationCreateInfo { memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
//    ..Default::default() },
//    verts).expect("Couldn't create vertex buffer")
//}
//
//pub fn begin_commandbuffer_recording(queue: Arc<Queue>, device: Arc<Device>)
//-> Result<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, Validated<VulkanError>>
//{
//
//    let cba = StandardCommandBufferAllocator::new(
//        device.clone(),
//        StandardCommandBufferAllocatorCreateInfo::default(),
//    );
//
//    AutoCommandBufferBuilder::primary(
//        &cba,
//        queue.clone().queue_family_index(),
//        CommandBufferUsage::OneTimeSubmit,
//    )
//}


