@target csharp

handler H {
    var a = @"brace } in verbatim";
    var b = $@"interp {1 + 2} with }} brace";
    var c = """raw quote with }""";
}

