@target typescript

native module runtime/socket {
    async frame_socket_client_connect(host: string, port: number) -> FrameSocketClient
    async frame_socket_client_read_line(instance: FrameSocketClient) -> string
    async frame_socket_client_write_line(instance: FrameSocketClient, line: string) -> void
    frame_socket_client_close(instance: FrameSocketClient) -> void
}

system RuntimeSocketDeclarationDemo {
    interface:
        run()

    machine:
        $Init {
            run() {
                var host = "127.0.0.1"
                var port = 7000
                var connect_promise = runtime_socket.frame_socket_client_connect(host, port)
                if connect_promise {
                    system.return = True
                } else {
                    system.return = False
                }
            }
        }
}

fn main() {
    var tester = RuntimeSocketDeclarationDemo()
    tester.run()
}
