@target python

system PyUnterminatedTripleQuote {
  actions:
    do_thing() {
      x = 1
      # Unterminated triple-quoted string below
      s = """this is not closed
    }

  interface:
  machine:
    $S {
      e -> $S
    }
  domain:
}

