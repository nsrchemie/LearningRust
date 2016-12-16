#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {

	let add_one = |x: i32| x + 1;
	assert_eq!(1,add_one(1));
}


pub fn add_two(a: i32) -> i32 {
a + 2
}

fn main() {

}
