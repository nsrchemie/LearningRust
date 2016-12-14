// use std::cell::Cell;

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

//USE OF MUTABLE POINTERS IN STRUCTS

	// struct Point {
	// 	x: i32,
	// 	y:i32,
	// }

	// struct PointRef<'a> {
	// 	x: &'a mut i32,
	// 	y: &'a mut i32,
	// }

	// let mut point = Point {x:0,y:0};

	// {
	// 	let r = PointRef{x: &mut point.x, y: &mut point.y};

	// 	*r.y = 6;
	// 	*r.x = 10;
	// }

	// println!("{} {}",point.x, point.y );


	//USE OF .. TO COPY PARTS OF STRUCTS FOR USE IN OTHER STRUCTS

	struct Point3d {
		x: i32,
		y: i32,
		z: i32,
	}

	let mut point = Point3d(x:0,y:0,z:0);
	point = Point3d {y:1, .. point};

}
