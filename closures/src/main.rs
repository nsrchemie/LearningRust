fn main() {


	//WORKING WITH CLOSURES
	//wrap up a function and free variables

	//simple closure 
	let plus_one = |x: i32| x + 1;
	println!("{}",plus_one(1));
}
