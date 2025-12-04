@target rust

// Bug #077 regression: comment-only terminal branch in event handler
// must still emit a statement so the generated Rust is compilable.
// @run-expect: Red

system EmptyElif {
  interface:
    run()

  machine:
    $Idle {
      run() {
        println!("Red");
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
  let mut tl = EmptyElif();
  tl.run();
  tl.run();
}
