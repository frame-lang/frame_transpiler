# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Python-specific Network external API test
# Must produce identical output to TypeScript version

import socket
import json
import threading
import time

system NetworkTest {
    actions:
        testNetworkOperations() {
            print("=== Network Test ===")
            
            # Create server socket using Python socket
            var server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            
            try {
                server_socket.bind(("127.0.0.1", 0))  # Let OS choose port
                var server_address = server_socket.getsockname()
                var port = server_address[1]
                server_socket.listen(1)
                print("Server created: true")
                print(f"Server listening on port: {port}")
                
                # Simulate client connection in separate thread
                var client_thread_obj = threading.Thread(target=client_thread, args=(port,))
                client_thread_obj.start()
                
                # Accept connection
                var conn, addr = server_socket.accept()
                print("Request received: true")
                
                # Send response with JSON
                var response_data = json.dumps({"status": "ok", "message": "Frame network test"})
                var http_response = f"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{response_data}"
                conn.send(http_response.encode())
                print(f"Response sent: {response_data}")
                
                conn.close()
                client_thread_obj.join()
                
            } finally {
                server_socket.close()
                print("Server closed: true")
            }
            
            print("=== Network Test Complete ===")
        }
}

fn main() {
    testNetworkOperations()
    return
}

fn client_thread(port) {
    time.sleep(0.1)  # Give server time to start
    var client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(("127.0.0.1", port))
    client_socket.send(b"GET /test HTTP/1.1\r\n\r\n")
    var response = client_socket.recv(1024)
    client_socket.close()
    return
}
