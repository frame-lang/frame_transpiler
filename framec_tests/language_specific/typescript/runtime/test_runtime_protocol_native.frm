@target typescript

#[target: typescript]
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
            #[target: typescript]
            {
                const socket = new Socket();
                await new Promise<void>((resolve, reject) => {
                    socket.once("connect", () => resolve());
                    socket.once("error", (err) => reject(err));
                    socket.connect({ host, port });
                });
                this.socket = socket;
            }
            return
        }

        async read_line() {
            if not self.socket {
                throw "Socket not connected"
            }
            var raw = ""
            #[target: typescript]
            {
                raw = await new Promise<string>((resolve, reject) => {
                    this.socket.once("data", (buffer) => resolve(buffer.toString("utf8")));
                    this.socket.once("error", (err) => reject(err));
                });
            }
            return raw
        }

        async write_line(line) {
            if not self.socket {
                throw "Socket not connected"
            }
            #[target: typescript]
            {
                await new Promise<void>((resolve, reject) => {
                    this.socket.write(line + "\n", (err) => {
                        if (err) {
                            reject(err);
                        } else {
                            resolve();
                        }
                    });
                });
            }
            return
        }

        disconnect() {
            if self.socket {
                #[target: typescript]
                this.socket.destroy()
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
