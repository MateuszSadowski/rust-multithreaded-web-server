use std::thread;

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    /// Create a new ThreadPool if size is larger than 0.
    /// Returns a ThreadPoolCreationError otherwise.
    /// 
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        if size <= 0 {
            return Err(ThreadPoolCreationError::new("size is smaller or equal 0".to_string()));
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f);
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
