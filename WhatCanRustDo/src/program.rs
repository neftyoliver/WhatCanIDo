use std::cmp::PartialEq;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::park;
use std::time::Duration;
use vulkano::swapchain::{Surface, Swapchain};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use winit::application::ApplicationHandler;
use winit::window::{Window, WindowAttributes};
use crate::program::UpdateMessage::NoProblem;
use crate::{program, rendering};
use crate::window::App;

pub enum UpdateMessage {
    NoProblem,
    Shutdown,
    Unknown
}

trait UpdateMethod {
    fn call();
}



pub struct Program {
    name: String,
    renderer: rendering::renderer::Renderer,
    app: App,
    thread_builder: Option<thread::Builder>,
}

impl Program {
    pub fn new(name: String) -> Program {
        println!("Program preparing...");

        let mut event_loop = EventLoop::new().unwrap();
        let mut application = App::new(WindowAttributes::default().with_title("Hello World!"));

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);

        let renderer = rendering::renderer::Renderer::new(&event_loop, 0);

        let thread_builder = thread::Builder::new().name(name.clone());

        println!("Program finished preparing");
        
        Program {
            name,
            renderer,
            app: application,
            thread_builder: Some(thread_builder),
        }
    }

    fn update(&mut self) -> UpdateMessage {
        NoProblem
    }

    pub fn run(&mut self) {
        println!("Program started");

        let mut prog = Rc::new(&mut self);

        let j = prog.thread_builder.take().unwrap().spawn(
            || {
                loop {
                    prog.update();
                }
            }
        ).expect("Program thread PANIC while starting.");

        j.join().unwrap();
    }
}
