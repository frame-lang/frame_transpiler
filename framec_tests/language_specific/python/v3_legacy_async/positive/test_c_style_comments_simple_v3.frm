@target python_3

# Port of legacy test_c_style_comments_simple.frm to V3 syntax.

fn test_comments() {
    # Simple comment-only function to ensure comments and spacing are preserved.
    x = 42
    print("X is: " + str(x))
}

fn main() {
    test_comments()
}

