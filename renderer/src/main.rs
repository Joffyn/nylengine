#![feature(trusted_random_access)]
#![feature(slice_pattern)]
extern crate core;

use winit::event_loop::EventLoop;
use crate::pipeline::app::App;

pub mod pipeline;



fn main()
{
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app);

}

