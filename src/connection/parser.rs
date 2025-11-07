use std::{io, path::{Path, PathBuf}};

/// Analiza la línea de solicitud HTTP.
pub fn parse_request(line: &str) -> Result<(&str, &str), io::Error> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() < 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Solicitud HTTP inválida"));
    }
    Ok((parts[0], parts[1]))
}

/// Construye la ruta absoluta del archivo solicitado.
pub fn resolve_path(root: &Path, path: &str) -> PathBuf {
    let clean = if path == "/" {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };
    root.join(clean)
}
