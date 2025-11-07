use std::{
    io::{self, Write},
    net::TcpStream,
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::Duration,
};

use crate::utils::{log, json_response};

// ✅ GLOBAL THREAD-SAFE (reemplaza static mut)
static RESULTS: OnceLock<Arc<Mutex<Vec<String>>>> = OnceLock::new();

/// Maneja endpoints REST `/ok`, `/bad`, `/fail`, `/async`, `/result`.
pub fn handle_special_endpoints(stream: &mut TcpStream, method: &str, path: &str) -> io::Result<bool> {
    if method != "GET" {
        return Ok(false);
    }

    match path {
        "/ok" => {
            json_response(stream, 200, "OK", r#"{"status":200,"message":"Todo bien 👍"}"#)?;
            log("✅ [200] /ok ejecutado");
            Ok(true)
        }

        "/bad" => {
            json_response(stream, 400, "Bad Request", r#"{"status":400,"error":"Solicitud incorrecta"}"#)?;
            log("⚠️ [400] /bad ejecutado");
            Ok(true)
        }

        "/fail" => {
            json_response(stream, 500, "Internal Server Error", r#"{"status":500,"error":"Error interno del servidor"}"#)?;
            log("💥 [500] /fail ejecutado");
            Ok(true)
        }

        // 🕒 Endpoint 202 Accepted (Asíncrono)
        "/async" => {
            // Inicializa una sola vez, de forma segura
            let results = RESULTS.get_or_init(|| Arc::new(Mutex::new(Vec::new()))).clone();

            let mut stream_clone = stream.try_clone()?;

            thread::spawn(move || {
                log("🕒 Procesando tarea asíncrona...");
                thread::sleep(Duration::from_secs(4));

                let nuevos = vec![
                    "Archivo procesado: informe_ventas.pdf",
                    "Archivo procesado: resumen_clientes.csv",
                    "Archivo procesado: reporte_financiero.xlsx",
                    "Archivo procesado: log_servidor.txt",
                ];

                {
                    let mut guard = results.lock().unwrap();
                    for r in nuevos {
                        guard.push(r.to_string());
                    }
                }

                let msg = "{\"status\":200,\"result\":\"Tarea asincrónica completada\"}".as_bytes();
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
                    msg.len()
                );
                let _ = stream_clone.write_all(header.as_bytes());
                let _ = stream_clone.write_all(msg);
                log("✅ Tarea asíncrona completada y resultados agregados");
            });

            json_response(stream, 202, "Accepted", r#"{"status":202,"message":"Tarea en proceso..."}"#)?;
            log("🟡 [202] /async lanzado");
            Ok(true)
        }

        // 📤 Endpoint /result — devuelve todos los resultados almacenados
        "/result" => {
            if let Some(results) = RESULTS.get() {
                let guard = results.lock().unwrap();
                let json = serde_json::to_string_pretty(&*guard).unwrap();
                json_response(stream, 200, "OK", &json)?;
                log("📤 [200] Resultados enviados desde memoria");
            } else {
                json_response(stream, 404, "Not Found", r#"{"error":"No hay resultados aún"}"#)?;
                log("⚠️ [404] Sin resultados disponibles");
            }
            Ok(true)
        }

        _ => Ok(false),
    }
}
