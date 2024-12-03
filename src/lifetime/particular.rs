pub fn foo<'a>(input: &'a str) -> &'static str {
    // "abc"
    let reference: &'static str =  "abc";
    reference
}