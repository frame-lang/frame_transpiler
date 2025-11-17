@target python_3

# Port of legacy test_python_fstring_nested_frame_statement_tokens.frm to V3 syntax.
# Purpose: ensure Frame tokens in f-strings are ignored by the scanner inside native code.

fn main() {
    a = 1
    name = "world"
    s1 = f"hello {name.upper()}"
    s2 = f"tokens -> $Next and $$[+] and => $^ inside fstring"
    inner = f"level2 {a}"
    s3 = f"nested {inner} end"
}
