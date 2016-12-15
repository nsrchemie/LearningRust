fn main() {

	//match is used for exhaustive conditionals

	// let x = 5;

	// match x {
	// 	1 => println!("One"),
	// 	2 => println!("Two"),
	// 	_ => println!("Value above 2"),
	// }

	//match can also be used in a let binding 
	//to convert between types 

	let number = match x {
		1 => "one",
		2 => "two",
		_=> "Other number",
	};

	println!("{}",number);
}
