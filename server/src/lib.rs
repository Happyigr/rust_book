use std::{
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The max_threads is the number of maximum threads in the server, that will equally working
    ///
    /// # Panics
    ///
    /// The new function panics if the function is zero
    pub fn new(max_threads: usize) -> ThreadPool {
        assert!(max_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(max_threads);

        for i in 0..max_threads {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    //in the doocumentation we find that spawn in the std thread is:
    //pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,

    pub fn execute<F>(&self, f: F)
    where
        // we use the () because this closure returns nothing and gets nothing
        F: Send + FnOnce() + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            drop(self.sender.take());

            println!("Shutting down the worker {}", worker.id);

            if let Some(thread) = worker.task.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    task: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let task = thread::spawn(move || loop {
            let mssg = receiver.lock().unwrap().recv();

            match mssg {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id}, disconnected; shutting down");
                    break;
                }
            }
        });

        Worker {
            id,
            task: Some(task),
        }
    }
}
