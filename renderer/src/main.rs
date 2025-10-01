use vulkano::memory::allocator::StandardMemoryAllocator;
use std::sync::Arc;
pub mod pipeline;

use crate::pipeline::instance::{self, create_device_and_queue};
use crate::pipeline::vertex::{TestVertex};

fn main() {
    //let (device, queue) = create_device_and_queue();

   //let memalloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    
    //let verts = vec![TestVertex{ pos:[rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)]}];
    



}

