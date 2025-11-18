@target typescript
// @skip-if: tsc-missing
// @tsc-compile
// @compile-expect: export class TsEmitIssues
// @compile-expect: public lifecycle: any =  "idle"
// @compile-expect: public last: any =  ""
// @compile-expect: public commandQueue: any =  \[]
// @compile-expect: public runtimeMessage\(
// @compile-expect: public handle\(payload
// @compile-expect: public enqueueCommand\(action, data
// @compile-expect: public flushCommands\(

system TsEmitIssues {
  interface:
    start()
    runtimeMessage(payload)

  machine:
    $S {
      start() { this.lifecycle = "starting" }
      runtimeMessage(payload) { this.handle(payload) }
    }
    $T {
      start() { this.enqueueCommand("x", {}) }
    }

  actions:
    handle(payload) { this.last = payload }
    enqueueCommand(action, data) { this.commandQueue.push({ action, data }) }
    flushCommands() { const q = this.commandQueue; this.commandQueue = []; return q }

  domain:
    lifecycle = "idle"
    last = ""
    commandQueue = []
}
