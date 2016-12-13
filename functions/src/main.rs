fn main() {
	let mut x = 17;
	println!("x is {}",x);
	x = my_func();
	println!("x is {}",x);
	x = calc(100, 2);
	println!("x is {}",x);
}

fn my_func() -> i32{
	let x = 9;
	let y = 10;
	y
}

fn calc(x:i32, y:i32) -> i32{
	let result = x * y;
	result
}