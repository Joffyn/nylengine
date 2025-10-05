use renderer::pipeline::window::create_default_window;

pub mod pipeline;
fn main() {
    //let (device, queue) = create_device_and_queue();

   //let memalloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    
    //let verts = vec![TestVertex{ pos:[rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5)]}];
    

    
    use crate::pipeline::window;

    create_default_window();



}

