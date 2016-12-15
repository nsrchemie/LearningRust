fn main() {
    // let greeting = "Hello!";
    //The above is a string literal and its type is
    //&'static str
    //string literals are statically allocated
    //they exist for the entire duration they run

    //two forms of strings spanning multiple lines
    let s = "foo
    bar";
    //the assert has the newline and 
    //any leading whitespace (4 spaces)
    assert_eq!("foo\n    bar", s);
}
