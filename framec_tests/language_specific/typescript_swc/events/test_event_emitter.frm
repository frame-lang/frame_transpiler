@target typescript

import { EventEmitter } from "events";

system EventsOnceDemo {
    machine:
        $Init {
            async start() {
                await self.demo()
                return
            }
        }

    actions:
        async demo() {
            const ee = new EventEmitter()
            await new Promise<void>((resolve, reject) => {
                ee.once("data", () => resolve())
                try {
                    ee.emit("data")
                } catch (e) {
                    reject(e)
                }
            })
            return
        }
}

