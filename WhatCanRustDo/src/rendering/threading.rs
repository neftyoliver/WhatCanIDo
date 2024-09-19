use std::thread;

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let thread = thread::spawn(move || {
            loop {

            }
        });

        Worker {
            id,
            thread
        }
    }
}



struct RenderingThreadPool {
    workers: Vec<Worker>,
}

impl RenderingThreadPool {
    fn new(size: u32) -> RenderingThreadPool {
        if size == 0 {
            panic!("Size should be greater than 0");
        }

        let mut workers = Vec::with_capacity(size as usize);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        RenderingThreadPool {
            workers
        }
    }
}