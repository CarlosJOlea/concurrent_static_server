use std::{
    io::{self, Write},
    net::TcpStream,
    path::Path,
    time::SystemTime,
};

/// Devuelve el tipo MIME según la extensión del archivo.
pub fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "txt" => "text/plain; charset=utf-8",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "application/octet-stream",
    }
}

/// Envía una respuesta HTTP con contenido JSON.
pub fn json_response(stream: &mut TcpStream, code: u16, status: &str, body: &str) -> io::Result<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
        code, status, body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body.as_bytes())?;
    stream.flush()
}

/// Imprime logs con timestamp UNIX.
pub fn log(msg: &str) {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    println!("[{ts}] {msg}");
}
