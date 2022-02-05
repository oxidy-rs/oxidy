use num_cpus;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/*
 * Thread Pool
 */
pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(mut size: usize) -> ThreadPool {
        if size < 1 {
            size = num_cpus::get();
            if size < num_cpus::get_physical() {
                size = num_cpus::get_physical();
            }
        }
        /*
         * Job Channel
         */
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        /*
         * Workers
         */
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub(crate) fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<F> = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
/*
 *  Worker Thread
 */
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job: Job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id, thread }
    }
}
/*
 * Job for Worker
 */
type Job = Box<dyn FnOnce() + Send + 'static>;
