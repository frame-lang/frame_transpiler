@target typescript
// @skip-if: tsc-missing
// @tsc-compile
// @compile-expect: export class AdapterProtocolMinimal
// @compile-expect: public start\(\): any
// @compile-expect: public runtimeConnected\(\): any
// @compile-expect: public runtimeMessage\(
// @compile-expect: public runtimeDisconnected\(\): any
// @compile-expect: public requestTerminate\(\): any
// @compile-expect: public drainCommands\(\): any

// Minimal AdapterProtocol-style fixture for V3 TS CLI tests.
// Mirrors the shared adapter_protocol_minimal.frm used in framepiler_test_env.

system AdapterProtocolMinimal {

  interface:
    start()
    runtimeConnected()
    runtimeMessage(payload)
    runtimeDisconnected()
    requestTerminate()
    drainCommands()

  machine:
    $Idle {
      start() {
        this.commandQueue = []
        this.handshakeComplete = false
        this.isReady = false
        this.pendingAction = false
        this.deferredQueue = []
        this.isPaused = false
        this.lastStoppedReason = ""
        this.lastThreadId = 0
        -> $WaitingForConnection
      }

      runtimeConnected() {
        // Ignore until started
      }

      runtimeMessage(payload) {
        // Ignore until started
      }

      runtimeDisconnected() { }

      requestTerminate() {
        // Nothing to terminate yet
      }

      drainCommands() {
        return this.flushCommands()
      }
    }

    $WaitingForConnection {
      runtimeConnected() {
        this.handshakeComplete = false
        this.enqueueCommand("initialize", {})
        this.enqueueCommand("ping", {})
        -> $Connected
      }

      runtimeMessage(payload) {
        // Ignore stray messages until connection confirmed
      }

      runtimeDisconnected() { }

      requestTerminate() {
        -> $Terminated
      }

      drainCommands() {
        return this.flushCommands()
      }
    }

    $Connected {
      runtimeMessage(payload) {
        this.handleConnectedMessage(payload)
      }

      runtimeDisconnected() { }

      requestTerminate() {
        // Minimal fixture: do not model terminating state; just drain.
      }

      drainCommands() {
        return this.flushCommands()
      }
    }

    $Terminated {
      start() { }
      runtimeConnected() { }
      runtimeMessage(payload) { }
      runtimeDisconnected() { }
      requestTerminate() { }
      drainCommands() {
        return this.flushCommands()
      }
    }

  actions:
    enqueueCommand(action, data) {
      const guarded: Record<string, boolean> = { continue: true, next: true, stepIn: true, stepOut: true, pause: true };
      const entry = { type: "command", action, data } as any;
      if (guarded[action]) {
        if (this.isReady !== true || this.handshakeComplete !== true) {
          if (Array.isArray(this.deferredQueue) && this.deferredQueue.some((e: any) => e && e.action === action)) {
            return;
          }
          this.deferredQueue.push(entry);
          return;
        }
        if (this.pendingAction === true) {
          return;
        }
        this.pendingAction = true;
      } else if (action === "setBreakpoints") {
        if (this.isReady !== true || this.handshakeComplete !== true) {
          if (Array.isArray(this.deferredQueue) && this.deferredQueue.some((e: any) => e && e.action === action)) {
            return;
          }
          this.deferredQueue.push(entry);
          return;
        }
      }
      this.commandQueue.push(entry);
    }

    flushCommands() {
      const queued = this.commandQueue;
      this.commandQueue = [];
      return queued;
    }

    handleConnectedMessage(payload) {
      const eventType = payload["event"];
      if (eventType === "hello") {
        this.handshakeComplete = true;
      } else if (eventType === "ready") {
        this.isReady = true;
        if (this.deferredQueue && this.deferredQueue.length > 0) {
          const guarded: Record<string, boolean> = { continue: true, next: true, stepIn: true, stepOut: true, pause: true };
          for (const e of this.deferredQueue) {
            if (!e || !e.action) {
              continue;
            }
            if (guarded[e.action]) {
              if (this.pendingAction === true) {
                continue;
              }
              this.pendingAction = true;
            }
            this.commandQueue.push(e);
          }
          this.deferredQueue = [];
        }
      } else if (eventType === "continued") {
        this.pendingAction = false;
        this.isPaused = false;
      } else if (eventType === "stopped") {
        this.pendingAction = false;
        this.isPaused = true;
        try {
          this.lastStoppedReason = payload["data"]["reason"];
          this.lastThreadId = payload["data"]["threadId"];
        } catch (_e) {
          // ignore
        }
      } else if (eventType === "terminated") {
        this.isPaused = false;
        this.commandQueue = [];
        this.deferredQueue = [];
      }
    }

  domain:
    commandQueue = []
    handshakeComplete = false
    isReady = false
    pendingAction = false
    deferredQueue = []
    isPaused = false
    lastStoppedReason = ""
    lastThreadId = 0
}
