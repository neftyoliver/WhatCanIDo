pub mod program {
    use vulkano::swapchain::{Surface, Swapchain};
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    };
    use crate::rendering as rd;

    pub struct Program {
        event_loop: EventLoop<()>,
        renderer: rd::renderer::Renderer,
    }

    impl Program {
    }
}