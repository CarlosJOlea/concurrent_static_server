use std::io::{self, Write};
use std::net::TcpStream;

/// Envía una respuesta HTTP genérica.
pub fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &[u8]) -> io::Result<()> {
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
        status,
        body.len(),
        content_type
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

/// Envía una respuesta de error genérica (HTML simple).
pub fn send_error(stream: &mut TcpStream, status: &str, body: &[u8]) -> io::Result<()> {
    respond(stream, status, "text/html; charset=utf-8", body)
}
