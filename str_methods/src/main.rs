fn main() {

	let mut x = "Random".to_string();
	//If we dont convert the static string slice to a String
	//we cant use string methods
	x.push_str(" String");
	println!("{}",x);
}
