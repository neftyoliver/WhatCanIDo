use std::ops::Deref;
use std::sync::Arc;
use std::thread;



/*

    * 쓰레드풀을 이용하려 멀티쓰레드 렌더링에 응용합니다.

*/

/*

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



#[derive(Clone)]
pub(crate) struct RenderingThreadPool {
    workers: Arc<[Worker]>,
}

impl RenderingThreadPool {
    pub fn new(size: u32) -> RenderingThreadPool {
        if size == 0 {
            panic!("Size should be greater than 0");
        }

        let mut workers = Vec::with_capacity(size as usize);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        let S: usize = size as usize;
        let mut arr_workers: [Worker; S] = [];

        for worker in arr_workers {
            arr_workers[worker.id as usize] = worker[worker.id as usize]
        }

        RenderingThreadPool {
            workers: Arc::new(arr_workers),
        }
    }


}

#[test]
fn main() {
    let thread_pool = RenderingThreadPool::new(4);


}
*/
