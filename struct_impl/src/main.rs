fn main() {

	//Demonstrating declaration of structs
	struct Rectangle {
		height: f64,
		width: f64
	}

	//we can define struct methods using impl
	impl Rectangle {
		//Creating a static method (Constructor), 
		//keep in mind the name 'new' can be anything
		fn new(h: f64, w: f64) -> 
		Rectangle {
			height: h,
			width: w
		}
	}

}

