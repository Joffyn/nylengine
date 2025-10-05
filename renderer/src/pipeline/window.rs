use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct Renderer
{
    window: Option<Window>,
}

impl ApplicationHandler for Renderer
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        self.window = Some(event_loop.create_window(Window::default_attributes()).expect("Couldn't create default window"));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent)
    {
        match event 
        {
            WindowEvent::CloseRequested => { println!("Window closing!"); event_loop.exit(); },
            WindowEvent::RedrawRequested => { self.window.as_ref().unwrap().request_redraw(); },
            _ => (),
        }
    }
}

 
pub fn create_default_window()
{
    let event_loop = EventLoop::new().expect("Couldn't create eventloop");
    let mut renderer = Renderer::default();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut renderer);
    //let def_attr = Window::default_attributes().with_title("Cool window");
    //event_loop.create_window(def_attr).expect("Couldn't create default window");
}


