use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use vulkano::swapchain::Surface;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::rendering;
use crate::rendering::renderer::Renderer;

#[derive(Clone)]
struct WindowObjects {
    attribute: WindowAttributes,
    rcptr_window: Arc<Window>
}

pub struct App {
    closed: bool,
    renderer: Arc<Renderer>,
    window_objects: Arc<WindowObjects>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let attribs = self.window_objects.clone().unwrap().attribute;

        self.rcptr_window = Arc::new(event_loop.create_window( attribs ).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

        match event {

            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                //Refreshing!

                self.rcptr_window.clone().as_ref().request_redraw();
            }

            _ => ()
        }
    }
}

impl App {
    pub fn new(attribute: WindowAttributes) -> App {
        println!("Creating application!");



        App {
            closed: false,
            renderer: Arc::default(),
            attribute,
            rcptr_window: Arc::default()
        }
    }

    fn run_app(&self) {
        let el = EventLoop::new().expect("Some thing going wrong with event loop creation.");

        let mut event_loop = Rc::new(el);

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);

        let renderer = Renderer::new(Surface::required_extensions(&event_loop.deref()), 0);

    }
}