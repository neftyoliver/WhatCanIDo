use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
pub struct App {
    attribute: Option<WindowAttributes>,
    window: Option<Window>,
    closed: bool
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let attrs = self.attribute.clone().unwrap().clone();

        self.window = Option::from(event_loop.create_window( attrs ).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {

        match event {

            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                //Refreshing!

                self.window.as_ref().unwrap().request_redraw();
            }

            _ => ()
        }
    }
}

impl App {
    pub fn new (attribute: WindowAttributes) -> App {
        App {
            attribute: Some(attribute),
            window: Default::default(),
            closed: false
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed()
    }
}