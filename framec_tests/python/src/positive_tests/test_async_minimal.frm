# Minimal test case for debugging async interface method parsing issue
# This should work but currently fails with "Expected 'fn' after 'async' keyword"
system Test {
    interface:
        async getData()
}