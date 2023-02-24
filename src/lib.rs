use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // TODO: Add explanation for parameters
    /// Create a new Worker.
    /// 
    /// # Panics
    ///
    /// The `new` function will panic if the mutex holding the receiver is poisoned or the sender
    /// stopped sending.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        let thread = Some(thread);
        
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of workers in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        Self::build(size).unwrap_or_else(|error| {
            match &error.kind() {
                ThreadPoolCreationErrorKind::BadArgument => { panic!("Thread pool creation bad argument error: {}", error.message); },
            }
        }) 
    }

    /// Create a new ThreadPool if size is larger than 0.
    /// Returns a ThreadPoolCreationError otherwise.
    /// 
    /// The size is the number of workers in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        if size <= 0 {
            return Err(ThreadPoolCreationError::new(ThreadPoolCreationErrorKind::BadArgument, "Size is smaller or equal 0.".to_string()));
        }

        let (workers, sender) = Self::create_workers(size);
        let sender = Some(sender);

        Ok(ThreadPool { workers, sender })
    }

    /// Sends a job to the pool of threads for execution.
    ///
    /// # Panics
    ///
    /// The `execute` function will panic if all the threads in the pool stop receiving messages.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // TODO: Handle errors
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    /// Creates the requested number of workers.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `create_workers` function will panic if the size is zero.
    fn create_workers(size: usize) -> (Vec<Worker>, mpsc::Sender<Job>) {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        (workers, sender)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Clone)]
pub enum ThreadPoolCreationErrorKind {
    BadArgument,
}

pub struct ThreadPoolCreationError {
    pub kind: ThreadPoolCreationErrorKind,
    pub message: String,
}

impl ThreadPoolCreationError {
    pub fn new(kind: ThreadPoolCreationErrorKind, message: String) -> ThreadPoolCreationError {
        Self {
            kind,
            message,
        }
    }

    pub fn kind(&self) -> ThreadPoolCreationErrorKind { self.kind.clone() }
}
