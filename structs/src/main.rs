use std::cell::Cell;

fn main() {
	//STRUCTS
	//useful for more complex data types

	// struct Point {
	// 	x: i32,
	// 	y: i32,
	// }
//create instance
// let origin = Point {x:0, y:0};

//create instance with mutability, you cant add mut to the definition
//of struct parameters like 
// Point {mut x: i32 --> THIS WILL NOT WORK
	//The below works
// 	let mut point = Point {x:0,y:0};
// 	point.x = 5;
// println!("x:{} y:{}",point.x,point.y);

//Can emulate field mutability by using Cell<T>

// struct Point {
// 	x: i32,
// 	y: Cell<i32>,
// }

// let point = Point {x: 5, y: Cell::new(6)};
// println!("y {:?}", point.y);
// point.y.set(7);

// println!("y has been changed to {:?}", point.y);


}
