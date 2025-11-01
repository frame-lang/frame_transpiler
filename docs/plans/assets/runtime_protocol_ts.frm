import json

# NOTE: Development fixture for Bug #055.
# Not wired into automated tests; serves as the TypeScript-specific
# runtime protocol blueprint while the async Node helpers are implemented.

system RuntimeProtocolTs {
    interface:
        async run(host, port)

    domain:
        socket

    machine:
        $Init {
            async run(host, port) {
                await self.connect(host, port)
                -> $Connected
            }
        }

        $Connected {
            async sendJson(value) {
                await self.write_line(json.dumps(value))
                return
            }

            async receiveJson() -> string {
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
                // resting terminal state
                return
            }
        }

    actions:
        async connect(host, port) {
            #[target: typescript]
            {
                const client = await FrameSocketClient.connect(host as string, Number(port));
                this.socket = client;
            }
            return
        }

        async read_line() -> string {
            #[target: typescript]
            {
                if (!this.socket) {
                    throw new Error("Socket not connected");
                }
                return await this.socket.readLine();
            }
            return ""
        }

        async write_line(line) {
            #[target: typescript]
            {
                if (!this.socket) {
                    throw new Error("Socket not connected");
                }
                await this.socket.writeLine(String(line));
            }
            return
        }

        disconnect() {
            #[target: typescript]
            {
                if (this.socket) {
                    this.socket.close();
                    this.socket = null;
                }
            }
            return
        }
    }
}

fn main() {
    var runtime = RuntimeProtocolTs()
    runtime.run("localhost", 9001)
}
