use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            receiver;
        });
        
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
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
