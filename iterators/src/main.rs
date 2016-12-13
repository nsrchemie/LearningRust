fn main() {

// Consumer, returns values with explicit type declaration for Vector
// let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
// Alternatively, can tell compiler to infer type of Vec
// let one_to_one_hundred = (1..101).collect::<Vec<_>>();
// let slice = &one_to_one_hundred[30..70];


//Find Consumer
let over_twenty = (0..80).find(|x| *x > 20);

//Without the below if we just printed the variable over_twenty
//We would receive an output of 'Some(21)', showing the first match
match over_twenty {
	Some(_) => println!("Met conditions"),
	None => println!("Conditions not met"),
}
}
