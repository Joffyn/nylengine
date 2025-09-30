use vulkano::VulkanLibrary;
use vulkano::memory::allocator::StandardMemoryAllocator;
use std::sync::Arc;
pub mod pipeline;

use crate::pipeline::instance::{self, create_device_and_queue};

fn main() {
    let (device, queue) = create_device_and_queue();

    let memalloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    
    



}

