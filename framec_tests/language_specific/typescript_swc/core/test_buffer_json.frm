@target typescript

import { Buffer } from "buffer";

system BufferJsonDemo {
    machine:
        $Init {
            async start() {
                await self.run()
                return
            }
        }

    actions:
        async run() {
            const val = JSON.parse("{\"a\":1}")
            const out = JSON.stringify(val)
            const buf = Buffer.from(out)
            const text = buf.toString()
            this.last = text
            return
        }

    domain:
        var last = ""
}

