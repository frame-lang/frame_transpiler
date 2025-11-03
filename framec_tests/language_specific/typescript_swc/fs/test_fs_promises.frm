@target typescript

import { promises as fs } from "fs";

system FsPromisesDemo {
    machine:
        $Init {
            async start() {
                await self.write_and_read()
                return
            }
        }

    actions:
        async write_and_read() {
            const path = "./out.txt"
            await fs.writeFile(path, "hello")
            const data = await fs.readFile(path, { encoding: "utf8" })
            this.last = data
            return
        }

    domain:
        var last = ""
}

