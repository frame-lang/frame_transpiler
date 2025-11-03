@target typescript

system PromisesArrowsDemo {
    machine:
        $Init {
            async start() {
                await self.wait_briefly()
                return
            }
        }

    actions:
        async wait_briefly() {
            // Arrow function + Promise pattern
            await new Promise<void>((resolve) => {
                setTimeout(() => resolve(), 1)
            })
            return
        }
}

