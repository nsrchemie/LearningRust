fn main() {
	//Strings are &str and String
	//&str are string slices
    // let greeting = "Hello!";
    //The above is a string literal and its type is
    //&'static str
    //string literals are statically allocated
    //they exist for the entire duration they run
    //They are NOT MUTABLE

    //two forms of strings spanning multiple lines
    // let s = "foo
    // bar";
    // //the assert has the newline and 
    // //any leading whitespace (4 spaces)
    // assert_eq!("foo\n    bar", s);


    // //You can trim spaces AND newline
    // let s = "foo\
    //           bar";

    // assert_eq!("foobar",s);


    //Accessing str's can only be done through &str reference
    //str is an unsized type, require addtl runtime info to 
    //be used
    
    //String is a heap-allocated string
    //This makes it growable, and guaranteed UTF-8
    //To create a String, convert a string  slice using
    //to_string method

    // let mut s = "Hello".to_string();
    // println!("{}",s);

    // s.push_str(", world");
    // println!("{}",s);

    //Can convert String intto &str with &
    fn take_slice(slice:&str) {
    	println!("{}",slice);
    }

    let s = "Hello".to_string();
    take_slice(&s);
}
