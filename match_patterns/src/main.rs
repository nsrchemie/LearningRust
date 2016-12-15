fn main() {

	//DESTRUCTURING

	struct Point {
		x: i32,
		y: i32,
	}

	let origin = Point {x: 0, y: 0};

	match origin {
		// Point { x, y } => println!("({}, {})",x,y),
		//Can also rename values
		// Point { x: x1, y: y1 } => println!("({}, {})",x1,y1),
		//You can selectively choose values
		Point {y, .. } => println!("y is {}",y),
	}
}
