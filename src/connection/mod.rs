use std::{
    fs,
    io::{self, BufRead, BufReader},
    net::TcpStream,
    path::Path,
    time::Duration,
};

use crate::utils::{log, mime_for};

mod parser;
mod responder;
mod endpoints;

use parser::{parse_request, resolve_path};
use responder::{respond, send_error};
use endpoints::handle_special_endpoints;

/// Maneja una conexión TCP entrante.
pub fn handle_connection(mut stream: TcpStream, root: &Path) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let (method, path) = parse_request(&request_line)?;

    // --- endpoints REST ---
    if handle_special_endpoints(&mut stream, method, path)? {
        return Ok(());
    }

    // --- archivos estáticos ---
    if method != "GET" {
        return send_error(&mut stream, "405 Method Not Allowed", b"<h1>405</h1>");
    }

    let file_path = resolve_path(root, path);
    match fs::read(&file_path) {
        Ok(content) => {
            let mime = mime_for(&file_path);
            respond(&mut stream, "200 OK", mime, &content)?;
            log(&format!("✅ 200 - {:?}", file_path.file_name().unwrap_or_default()));
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            send_error(&mut stream, "404 Not Found", b"<h1>404</h1>")?;
            log(&format!("⚠️ 404 - {:?}", file_path.file_name().unwrap_or_default()));
        }
        Err(e) => {
            send_error(&mut stream, "500 Internal Server Error", b"<h1>500</h1>")?;
            log(&format!("💥 500 - {e}"));
        }
    }

    Ok(())
}
