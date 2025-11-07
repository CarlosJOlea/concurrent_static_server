use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Pool de hilos sencillo para manejar tareas concurrentes.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Option<Job>>,
}

impl ThreadPool {
    /// Crea un nuevo pool con `size` hilos.
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "Debe haber al menos un hilo");
        let (sender, receiver) = mpsc::channel::<Option<Job>>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_| Worker::new(Arc::clone(&receiver)))
            .collect();

        Self { workers, sender }
    }

    /// Envía un trabajo al pool.
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Some(Box::new(job)));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            let _ = self.sender.send(None);
        }
        for w in &mut self.workers {
            if let Some(handle) = w.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Option<Job>>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = {
                let rx = receiver.lock().unwrap();
                rx.recv()
            };

            match job {
                Ok(Some(job)) => job(),
                Ok(None) | Err(_) => break,
            }
        });

        Self { handle: Some(handle) }
    }
}
