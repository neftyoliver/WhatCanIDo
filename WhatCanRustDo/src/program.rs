use std::sync::Arc;
use std::thread;
use vulkano::swapchain::{Surface, Swapchain};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use winit::window::Window;
use crate::rendering;

#[derive(Default)]
pub struct Program {
    app: Option<Window>,
    event_loop: Arc<EventLoop<()>>,
    renderer: rendering::renderer::Renderer,
}

impl Program {
    pub fn new() -> Program {
        let app = Default::default();

        let event_loop = EventLoop::new().unwrap();

        let renderer = rendering::renderer::Renderer::new(&event_loop, 0);

        Program {
            app,
            event_loop: Arc::new(event_loop),
            renderer
        }
    }

    pub fn start(&mut self) {
        let mut arc_window = self.event_loop.run_app()

        let jh = thread::spawn(|| {
            let window = winit::window::WindowBuilder

        });

        jh.join()
    }
}