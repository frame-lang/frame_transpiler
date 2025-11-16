@target typescript
// @skip-if: tsc-missing
// @tsc-compile
// (No compile-expect patterns: this fixture is focused on preventing TS2451
// from redeclaring transition temporaries across multiple cases.)

system RedeclareVar {
  interface:
    runtimeDisconnected()

  machine:
    $A { runtimeDisconnected() { -> $B } }
    $B { runtimeDisconnected() { -> $Terminated } }
    $Terminated { runtimeDisconnected() { /* ignore */ } }
}
