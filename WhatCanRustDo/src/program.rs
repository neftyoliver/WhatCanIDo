
pub mod prog {
    use std::sync::Arc;
    use std::thread;
    use vulkano::swapchain::Surface;
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::WindowAttributes;
    use crate::rendering::renderer::Renderer;

    mod win {
        use std::ops::Deref;
        use std::sync::Arc;
        use winit::application::ApplicationHandler;
        use winit::event::WindowEvent;
        use winit::event_loop::ActiveEventLoop;
        use winit::window::{Window, WindowAttributes, WindowId};
        use crate::rendering::renderer::Renderer;

        pub struct AppHandler {
            pub(crate) renderer: Arc<Renderer>,
            pub(crate) attribute: Arc<WindowAttributes>,
            pub(crate) window: Option<Window>
        }

        impl ApplicationHandler for AppHandler {

            fn resumed(&mut self, event_loop: &ActiveEventLoop) {
                let attr = self.attribute.clone().as_ref().clone();

                self.window = Some(event_loop.create_window(attr).unwrap());
            }

            fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
                match event {
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                    }

                    WindowEvent::RedrawRequested => {

                        self.window.as_ref().clone().expect("No window to redraw.").request_redraw();

                    }

                    _ => {}
                }


            }
        }
    }

    pub struct Program {
        pub(crate) window_attributes: Arc<WindowAttributes>,
    }

    impl Program {
        pub fn start(&mut self) {
            let event_loop = EventLoop::new().unwrap();
            event_loop.set_control_flow(ControlFlow::Poll);

            let renderer = Arc::new(Renderer::new(Surface::required_extensions(&event_loop), 0));
            let mut handler = win::AppHandler {
                renderer,
                attribute: self.window_attributes.clone(),
                window: None
            };

            event_loop.run_app(&mut handler).unwrap();
        }
    }
}