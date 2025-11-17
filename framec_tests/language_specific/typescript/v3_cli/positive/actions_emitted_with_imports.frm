@target typescript
// @compile-expect: class RuntimeProtocol

system RuntimeProtocol {
    interface:
        run()
    machine:
        $Idle {
            run() {
                // interface handler
            }
        }
    actions:
        // import inside actions: should not be treated as a top-level module import
        handleCommand(message: any) {
            import * as fs from "fs";
            const action = message.action;
            if (action === "ping") {
                await this.sendOutput("TS pong\\n", "stdout");
            }
        }
}
