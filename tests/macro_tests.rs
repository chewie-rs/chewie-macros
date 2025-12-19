#[test]
fn test_expansions() {
    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn test_compile_fail() {
    macrotest::expand("tests/expand-fail/*.rs");
}
