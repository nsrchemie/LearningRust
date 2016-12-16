fn main() {


	//WORKING WITH CLOSURES
	//wrap up a function and free variables

	//simple closure 
	// let plus_one = |x: i32| x + 1;
	// println!("{}",plus_one(1));

	//Multi-line closure
	// let plus_two = |x| {
	// 	let mut result: i32 = x;

	// 	result += 1;
	// 	result += 1;

	// 	result
	// };

	// println!("{}",plus_two(1));

	//You can simplify closures
	let plus_one = |x: i32| -> i32 {x + 1};
	println!("{}", plus_one(1));
}
