use std::sync::Arc;
use std::thread;
use winit::window::WindowAttributes;

mod rendering;
mod program;

fn main() {
    let mut prog = program::prog::Program {
        window_attributes: Arc::new(WindowAttributes::default().with_visible(true))
    };

    prog.start();
}
