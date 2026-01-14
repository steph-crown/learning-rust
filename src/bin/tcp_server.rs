// --- Main Entry Point ---

use prog_concepts::tcp_server::TcpServer;

fn main() {
  let server = TcpServer::new("127.0.0.1:5001");
  server.run();
}
