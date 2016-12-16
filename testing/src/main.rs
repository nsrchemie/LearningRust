// #[test]
// #[should_panic(expected = "assertion failed")]
// fn it_works() {

// 	let add_one = |x: i32| x + 1;
// 	assert_eq!(1,add_one(1));
// }


pub fn add_two(a: i32) -> i32 {
a + 2
}

#[cfg(test)]
//module allows us to group together tests
//cfg attr compiles test if we want to run test code
//use super brings test function into scope
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(4,add_two(2));
	}
}



fn main() {

}
