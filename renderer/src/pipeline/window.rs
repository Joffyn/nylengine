//use winit::application::ApplicationHandler;
//use winit::event::WindowEvent;
//use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
//use winit::window::{Window, WindowId};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use std::sync::Arc;



//#[derive(Default)]
//pub struct Renderer
//{
//    pub window: Option<Window>,
//}
//
//impl ApplicationHandler for Renderer
//{
//    fn resumed(&mut self, event_loop: &ActiveEventLoop)
//    {
//        self.window = Some(event_loop.create_window(Window::default_attributes()).expect("Couldn't create default window"));
//    }
//
//    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent)
//    {
//        match event 
//        {
//            WindowEvent::CloseRequested => { println!("Window closing!"); event_loop.exit(); },
//            WindowEvent::RedrawRequested => { self.window.as_ref().unwrap().request_redraw(); },
//            _ => (),
//        }
//    }
//}


 
pub fn create_default_window() -> Arc<Window>
{
    let event_loop = EventLoop::new();
    let window = Arc::new(Window::new(&event_loop).expect("Couldn't create window!"));

    event_loop.run(move |event, _, control_flow| 
    {
        match event 
        {
            Event::WindowEvent 
                { 
                event: WindowEvent::CloseRequested, .. } => 
            { 
                println!("Window closing!");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => { window.clone().request_redraw();},
            _ => (),
        }
    });
    window
}


