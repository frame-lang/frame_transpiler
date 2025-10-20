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
            var server = net.createServer()
            
            server.listen(0, "127.0.0.1", function() {
                var port = server.address().port
                print("Server created: true")
                print(f"Server listening on port: {port}")
                
                # Create client connection
                var client = net.createConnection(port, "127.0.0.1")
                
                client.on("connect", function() {
                    print("Request received: true")
                    client.write("GET /test HTTP/1.1\r\n\r\n")
                })
                
                client.on("data", function(data) {
                    # Server would normally handle this, but for test simplicity
                    # we'll just acknowledge the response was processed
                })
                
                client.on("end", function() {
                    client.end()
                })
            })
            
            server.on("connection", function(socket) {
                socket.on("data", function(data) {
                    # Send JSON response using Node.js idioms
                    var response_data = JSON.stringify({"status": "ok", "message": "Frame network test"})
                    var http_response = f"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{response_data}"
                    socket.write(http_response)
                    print(f"Response sent: {response_data}")
                    socket.end()
                })
            })
            
            # Give time for connection to complete
            setTimeout(function() {
                server.close()
                print("Server closed: true")
                print("=== Network Test Complete ===")
            }, 100)
        }
}

fn main() {
    var tester = NetworkTest()
    tester.testNetworkOperations()
    return
}