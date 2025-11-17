@target python_3

# Port of legacy test_python_triple_quote_with_frame_statements.frm to V3 syntax.
# Purpose: ensure triple-quoted strings containing Frame tokens do not segment bodies.

fn main() {
    doc = """
    This triple-quoted string contains tokens that must not segment:
    -> $Next
    $$[+]
    => $^
    """
    x = len(doc)
    print("Length:", x)
}
