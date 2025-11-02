@target typescript

#[target: typescript]
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
            #[target: typescript]
            {
                const socket = new Socket();
                this.socket = socket;
                await new Promise<void>((resolve, reject) => {
                    socket.once("connect", () => resolve());
                    socket.once("error", (err) => reject(err));
                    socket.connect({ host, port });
                });
                socket.destroy();
            }
            return
        }

    domain:
        var socket = null
}

fn main() {
    var demo = RuntimeSocketImportDemo()
    demo.connect("127.0.0.1", 7000)
}
