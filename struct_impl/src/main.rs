fn main() {

	//Demonstrating declaration of structs
	struct Rectangle {
		height: f64,
		width: f64
	}

	//we can define struct methods using impl
	impl Rectangle {
		//create structure method
		//notice we want to avoid moving, and instead reference
		fn area(&self) -> f64 {
			self.height * self.width
		}
	}

	let a_box = Rectangle {height: 3.2, width: 6.9};
	println!("Area on my rectangle is {:?}",a_box.area());

}

