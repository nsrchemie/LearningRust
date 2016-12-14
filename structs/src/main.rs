fn main() {
	//STRUCTS
	//useful for more complex data types

	struct Point {
		x: i32,
		y: i32,
	}
//create instance
let origin = Point {x:0, y:0};
println!("x:{} y:{}",origin.x,origin.y);
}
