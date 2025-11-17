@target python_3

# Port of legacy test_mixed_body_whitespace_tabs_spaces.frm to V3 syntax.
# Purpose: ensure tabs + spaces and mixed indentation do not confuse Frame scanners.

system TabsSpacesPyV3 {
    machine:
        $Init {
            start() {
\t\t# tabs + spaces above
\t\tval = 1
                -> $Next
            }
        }
        $Next {}
}

