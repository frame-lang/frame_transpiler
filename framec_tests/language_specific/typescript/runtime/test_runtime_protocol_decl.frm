@target typescript

system RuntimeProtocolTs {
    interface:
        async run(host, port)

    machine:
        $Init {
            async run(host, port) {
                await self.connect(host, port)
                -> $Connected
            }
        }

        $Connected {
            async sendJson(value) {
                await self.write_line(value)
                return
            }

            async receiveJson() {
                var raw = await self.read_line()
                return raw
            }

            close() {
                self.disconnect()
                -> $Terminated
            }
        }

        $Terminated {
            terminate() {
                return
            }
        }

    actions:
        async connect(host, port) {
            var client = await runtime_socket.frame_socket_client_connect(host, port)
            self.socket = client
            return
        }

        async read_line() {
            if not self.socket {
                throw "Socket not connected"
            }
            var raw = await runtime_socket.frame_socket_client_read_line(self.socket)
            return raw
        }

        async write_line(line) {
            if not self.socket {
                throw "Socket not connected"
            }
            await runtime_socket.frame_socket_client_write_line(self.socket, line)
            return
        }

        disconnect() {
            if self.socket {
                runtime_socket.frame_socket_client_close(self.socket)
                self.socket = null
            }
            return
        }

    domain:
        var socket = null
}

fn main() {
    var runtime = RuntimeProtocolTs()
    runtime.run("localhost", 9001)
}
