use concurrent_static_server::WebServer;

fn main() -> std::io::Result<()> {
    WebServer::new("127.0.0.1:8080", "./static", 4).run()
}
