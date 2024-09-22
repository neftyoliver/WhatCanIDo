use std::ops::{Add, Deref, DerefMut};
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::window::App;
use crate::rendering;
use std::thread;
use vulkano::swapchain::Surface;
use winit::application::ApplicationHandler;
use winit::window::WindowAttributes;
use winit::event_loop::{ControlFlow, EventLoop};

enum UpdateResult {
    NoProblem,
    exit,
    Unknown
}
pub struct Program {
    name: String,
    app: Rc<App>,

}

impl Program {
    pub fn new(name: String) -> Program {
        println!("Program preparing...");
        let app = App::new(Default::default());

        Program {
            name,
            app: Rc::new(app),
        }
    }

    fn start(&mut self) {

    }

}
