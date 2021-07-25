pub struct ThreadPool;

impl ThreadPool {
    #[allow(unused_variables)]
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    #[allow(unused_variables)]
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
