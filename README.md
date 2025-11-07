# 🦀 Servidor Web Concurrente en Rust

## 📖 Descripción

Servidor web multihilo hecho en **Rust** que sirve archivos estáticos desde un directorio.  
Implementa concurrencia con **hilos**, manejo básico de **solicitudes HTTP**, **tipos MIME** y **errores comunes**.

### 🚀 Ejecución rápida

```bash
cargo build --release
cargo run -- 127.0.0.1:7878 ./static_files
```

Asegúrate de tener un directorio `static_files` con archivos como `index.html`, `style.css`, etc.  
Luego abre en el navegador:

👉 [http://127.0.0.1:7878/](http://127.0.0.1:7878/)

---

## 🌐 Endpoints disponibles

| Endpoint | Descripción |
|-----------|--------------|
| `/ok`     | Respuesta JSON **200 OK** |
| `/bad`    | Respuesta **400 Bad Request** |
| `/fail`   | Respuesta **500 Internal Server Error** |
| `/async`  | Lanza tarea asíncrona simulada |
| `/result` | Devuelve resultados de tareas |

---

## ⚙️ Concurrencia

El servidor usa un **ThreadPool** para manejar múltiples conexiones simultáneas de forma eficiente:

```rust
let server = WebServer::new("127.0.0.1:7878", "./static_files", 4);
server.run().unwrap();
```

---

## 🧪 Prueba rápida

Ejecuta el servidor y prueba en el navegador:

- [http://127.0.0.1:7878/ok](http://127.0.0.1:7878/ok)  
- [http://127.0.0.1:7878/async](http://127.0.0.1:7878/async)  
- [http://127.0.0.1:7878/result](http://127.0.0.1:7878/result)  

---

## 📚 Resumen de Preguntas y Respuestas del Proyecto “Servidor Web Concurrente en Rust”

### 🟢 1. ¿Podemos hacer una RESTful API con Rust y Cargo?

✅ **Sí.** Puedes crear una API REST usando la librería estándar (`std::net`, `std::thread`, `std::io`) o frameworks como **Axum** o **Actix-Web**.  
En este proyecto se construyó solo con `std` para entender desde cero cómo funciona un servidor HTTP concurrente.

---

### 🟢 2. ¿Podemos leer una lista de nombres de un archivo de forma asíncrona y en cola?

✅ **Sí.** Se usan **hilos (`thread::spawn`)** y una **cola compartida (`Arc<Mutex<VecDeque>>`)** que permite procesar trabajos en paralelo sin bloquear peticiones.

---

### 🟢 3. ¿Cumple con el enunciado del servidor concurrente en Rust?

✅ **Sí.** Implementa:
- Múltiples conexiones concurrentes (ThreadPool).  
- Lectura y respuesta HTTP manual.  
- Envío de archivos con su MIME correcto.  
- Manejo de errores 404, 405 y 500.

---

### 🟢 4. Error: “link.exe not found”

💡 **Solución:** Instala **Visual Studio Build Tools 2022** con el componente “Desarrollo de escritorio con C++”.  
Esto agrega el `link.exe` necesario para compilar en Windows (`x86_64-pc-windows-msvc`).

---

### 🟢 5. Error: “The system cannot find the file specified”

💡 **Causa:** La carpeta `static/` estaba dentro de `src/`.  
✅ **Solución:** Muévela a la raíz del proyecto, al mismo nivel que `Cargo.toml`.

---

### 🟢 6. ¿Podemos refactorizar el servidor?

✅ **Sí.** Se separó el código en módulos:

| Archivo | Función |
|----------|----------|
| `parser.rs` | Analiza solicitudes |
| `responder.rs` | Envía respuestas |
| `endpoints.rs` | Maneja rutas `/ok`, `/bad`, `/fail`, `/async` |
| `mod.rs` | Coordina todo |

Esto mejora la **organización, escalabilidad y legibilidad**.

---

### 🟢 7. Error: “non-ASCII character in byte string literal”

💡 **Causa:** Las cadenas `b"..."` solo aceptan ASCII.  
✅ **Solución:**
```rust
let msg = "{\"texto\":\"Hola ✅\"}".as_bytes();
```

---

### 🟢 8. ¿Por qué `/async` responde sin esperar?

💡 Porque cada `thread::spawn` responde sin bloqueo.  
✅ Para una cola real, se necesita un **`VecDeque`** con **`Arc<Mutex<_>>`** y un hilo que consuma trabajos secuencialmente.

---

### 🟢 9. ¿Podemos devolver un arreglo JSON ficticio después de un tiempo?

✅ **Sí.**  
Ejemplo:
```rust
let resultados = vec!["archivo1", "archivo2"];
serde_json::to_string(&resultados).unwrap();
```

---

### 🟢 10. ¿Por qué `/async` manda 202 pero no el JSON final?

💡 **Causa:** HTTP solo permite una respuesta por solicitud.  
✅ **Solución:** Guardar el resultado en un archivo (`static/result.json`) o en memoria compartida y consultarlo con `/result`.

---

### 🟢 11. ¿Qué pasa si lanzo varias peticiones `/async`?

💡 Cada hilo sobreescribe `result.json`.  
✅ Solución: generar nombres únicos o usar un vector compartido con `Arc<Mutex<Vec>>` para acumular resultados.

---

### 🟢 12. ¿Podemos usar `static mut RESULTS`?

💡 Ya no es permitido en Rust 1.91+.  
✅ Solución:
```rust
static RESULTS: OnceLock<Arc<Mutex<Vec>>> = OnceLock::new();
```

Seguro, concurrente y compatible con **Rust 2024**.

---

### 🟢 13. Error: “shared reference to mutable static”

💡 Ocurre por el uso de `static mut`.  
✅ Solución: reemplazarlo con **`OnceLock`**, que inicializa una variable global una sola vez y permite acceso seguro entre hilos.

---

### 🟢 14. ¿Podemos agregar un endpoint `/result`?

✅ **Sí.** Ejemplo:
```rust
let guard = RESULTS.get().unwrap().lock().unwrap();
serde_json::to_string_pretty(&*guard).unwrap();
```

Devuelve un JSON con todos los resultados acumulados.
