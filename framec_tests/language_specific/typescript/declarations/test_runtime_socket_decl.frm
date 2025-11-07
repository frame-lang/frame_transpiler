 
import { Socket } from "net";

system RuntimeSocketImportDemo {
    interface:
        async connect(host, port)

    machine:
        $Init {
            async connect(host, port) {
                await self.open_socket(host, port)
                return
            }
        }

    actions:
        async open_socket(host, port) {
            var endpoint = host + ":" + port
            self.endpoints = self.endpoints + [endpoint]
            return
        }

    domain:
        var endpoints = []
}

fn main() {
    var demo = RuntimeSocketImportDemo()
    demo.connect("127.0.0.1", 7000)
}
