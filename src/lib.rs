use std::thread;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(||{});
        
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
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
        let workers = Self::create_workers(size);

        ThreadPool { workers }
    }

    /// Create a new ThreadPool if size is larger than 0.
    /// Returns a ThreadPoolCreationError otherwise.
    /// 
    /// The size is the number of workers in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        if size <= 0 {
            return Err(ThreadPoolCreationError::new("size is smaller or equal 0".to_string()));
        }

        let workers = Self::create_workers(size);

        Ok(ThreadPool { workers })
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
    fn create_workers(size: usize) -> Vec<Worker> {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        workers
    }
}

pub struct ThreadPoolCreationError {
    pub message: String,
}

impl ThreadPoolCreationError {
    pub fn new(message: String) -> ThreadPoolCreationError {
        Self {
            message,
        }
    }
}
