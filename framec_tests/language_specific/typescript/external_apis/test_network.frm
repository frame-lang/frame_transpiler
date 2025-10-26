# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# TypeScript-specific Network external API test
# Must produce identical output to Python version

import net
import http

system NetworkTest {
    actions:
        testNetworkOperations() {
            print("=== Network Test ===")
            
            # Create server using Node.js net module
            self.server = net.createServer()
            FrameRuntime.addListener(self.server, "connection", self, "handle_connection")
            FrameRuntime.listen(self.server, 0, "127.0.0.1", self, "on_server_ready")
            FrameRuntime.setTimeout(self, "shutdown_server", 100)
        }

        on_server_ready() {
            var address = self.server.address()
            var port = FrameRuntime.extractPort(address)
            print("Server created: true")
            print(f"Server listening on port: {port}")
            
            # Create client connection
            self.client = net.createConnection(port, "127.0.0.1")
            FrameRuntime.addListener(self.client, "connect", self, "handle_client_connect")
            FrameRuntime.addListener(self.client, "data", self, "handle_client_data")
            FrameRuntime.addListener(self.client, "end", self, "handle_client_end")
        }

        handle_client_connect() {
            print("Request received: true")
            self.client.write("GET /test HTTP/1.1\r\n\r\n")
        }

        handle_client_data(data) {
            # Client just discards data in this test
        }

        handle_client_end() {
            self.client.end()
        }

        handle_connection(socket) {
            self.last_socket = socket
            FrameRuntime.addListener(socket, "data", self, "handle_socket_data")
        }

        handle_socket_data(data) {
            var response_data = JSON.stringify({"status": "ok", "message": "Frame network test"})
            var http_response = f"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{response_data}"
            self.last_socket.write(http_response)
            print(f"Response sent: {response_data}")
            self.last_socket.end()
        }

        shutdown_server() {
            self.server.close()
            print("Server closed: true")
            print("=== Network Test Complete ===")
        }

    domain:
        var server = None
        var client = None
        var last_socket = None
}

fn main() {
    var tester = NetworkTest()
    tester.testNetworkOperations()
    return
}
