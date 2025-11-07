use std::{
    fs,
    io,
    net::TcpListener,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::{connection::handle_connection, thread_pool::ThreadPool, utils::log};

/// Servidor HTTP concurrente que sirve archivos estáticos.
pub struct WebServer {
    addr: String,
    root: PathBuf,
    threads: usize,
}

impl WebServer {
    pub fn new<A: Into<String>, P: Into<PathBuf>>(addr: A, root: P, threads: usize) -> Self {
        Self {
            addr: addr.into(),
            root: root.into(),
            threads: threads.max(1),
        }
    }

    /// Ejecuta el servidor de forma bloqueante.
    pub fn run(self) -> io::Result<()> {
        self.run_with_shutdown(Arc::new(AtomicBool::new(false)))
    }

    /// Versión controlada (para pruebas), permite detener el servidor.
    pub fn run_with_shutdown(self, shutdown: Arc<AtomicBool>) -> io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        let root = fs::canonicalize(&self.root)?;
        let pool = ThreadPool::new(self.threads);

        listener.set_nonblocking(true)?;
        log(&format!("🚀 Servidor escuchando en http://{}", self.addr));

        while !shutdown.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let root = root.clone();
                    pool.execute(move || {
                        if let Err(e) = handle_connection(stream, &root) {
                            log(&format!("❌ Error: {}", e));
                        }
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(20));
                }
                Err(e) => return Err(e),
            }
        }

        log("🛑 Servidor detenido.");
        Ok(())
    }
}
