@target typescript

// Bug #077 regression: comment-only terminal branch in event handler
// must still emit a `;` so the generated TS is importable.
// @run-expect: Red

system EmptyElif {
  interface:
    run()

  machine:
    $Idle {
      run() {
        console.log("Red")
        -> $Terminated()
      }
    }

    $Terminated {
      run() {
        // Already terminated
      }
    }
}

fn main() {
  const tl = new EmptyElif();
  tl.run();
  tl.run();
}
