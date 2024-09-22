use std::thread;
use crate::program::UpdateMessage;

mod rendering;
mod program;
mod window;

fn main() {
    let mut prog = program::Program::new(String::from("Nefty's program!"));

    prog.run();

}
