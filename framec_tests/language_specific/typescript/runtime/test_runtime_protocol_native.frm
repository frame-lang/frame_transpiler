 
import { Socket } from "net";

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
            var endpoint = host + ":" + port
            self.socket = endpoint
            return
        }

        async read_line() {
            if not self.socket {
                throw "Socket not connected"
            }
            return self.socket
        }

        async write_line(line) {
            if not self.socket {
                throw "Socket not connected"
            }
            self.last_payload = line
            return
        }

        disconnect() {
            if self.socket {
                self.socket = null
            }
            return
        }

    domain:
        var socket = null
        var last_payload = ""
}

fn main() {
    var runtime = RuntimeProtocolTs()
    runtime.run("localhost", 9001)
}
