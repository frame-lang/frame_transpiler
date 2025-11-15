@target python_3
# @compile-expect: async def _action_runtimeMain
# @compile-expect: def _action_readConfigPort

system IndentRuntime {
    machine:
        $Idle {
            async run() {
                await self.runtimeMain()
                -> $Terminated
            }
        }

        $Terminated {
            run() { pass }
        }

    actions:
        async runtimeMain() {
            try:
                port = self.readConfigPort()
                await self.sendOutput(f"Port={port}\n", "stdout")
            except Exception as error:
                await self.sendOutput(f"Error: {error}\n", "stderr")
        }

        readConfigPort() {
            return 1234
        }
}

