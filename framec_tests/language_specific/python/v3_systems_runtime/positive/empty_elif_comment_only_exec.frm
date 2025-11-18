@target python_3

# Bug #077 regression: comment-only terminal branch in event handler
# must still emit a `pass` so the generated Python is importable.
# @run-expect: Red

system EmptyElif {
  interface:
    run()

  machine:
    $Idle {
      run() {
        print("Red")
        -> $Terminated()
      }
    }

    $Terminated {
      run() {
        # Already terminated
      }
    }
}

fn main() {
  tl = EmptyElif()
  tl.run()
  tl.run()
}

