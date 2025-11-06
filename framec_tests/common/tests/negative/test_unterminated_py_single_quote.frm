@target python

system PyUnterminatedSingleQuote {
  actions:
    do_thing() {
      # Unterminated single-quoted string
      s = 'not closed
    }

  interface:
  machine:
    $S {
      e -> $S
    }
  domain:
}

