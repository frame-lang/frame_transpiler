@target python_3

# Port of legacy test_c_style_comments.frm to V3 syntax.

system CommentDemo {
    actions:
        run() {
            # Single-line Python comment
            x = 1  # inline comment

            /* C-style block comments should be ignored by scanners */
            /* nested */ x = x + 1

            // C++ style comment
            x = x + 2
        }
}

