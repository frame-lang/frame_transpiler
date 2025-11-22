@target typescript
// @skip-if: tsc-missing
// @tsc-compile
// @compile-expect: export class DemoTsGen
// @compile-expect: public start\(\): any
// @compile-expect: public runtimeMessage\(
// @compile-expect: switch \(c\.state\)
// @compile-expect: case '__DemoTsGen_state_A':
// @compile-expect: case '__DemoTsGen_state_B':

system DemoTsGen {
  interface:
    start()
    runtimeMessage(payload)

  machine:
    $A {
      start() { /* no-op */ }
      runtimeMessage(payload) { /* ignore */ }
    }
    $B {
      start() { /* still no-op */ }
      runtimeMessage(payload) { /* also ignore */ }
    }

  actions:
    // no actions required for this duplication bug
}
