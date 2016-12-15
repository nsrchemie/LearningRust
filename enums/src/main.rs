fn main() {
	//enums are useful for mixing datatypes 
	//and variants dont necessarily need data associated
	//Unlike structs, enums are a single type
	enum Message {
		Quit,
		ChangeColor(i32, i32, i32),
		Move {x: i32, y: i32 },
		Write(String),
	}

	//working with enums
}
