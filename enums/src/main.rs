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

	let x: Message = Message::Move { x: 3, y: 4};

	enum BoardGameTurn {
		Move {squares: i32},
		Pass,
	}

	let y: BoardGameTurn = BoardGameTurn::Move { squares: 1};
	//Both enums have variants named Move but they are scoped individually and dont clash

	//Can use enum constructor like a function
	let m  = Message::Write("Hello!".to_string());
	//Equivalent code is
	//fn m(x: String) -> Message {
	// 		Message::Write(x)
	// }
	//let x = m("Hello!".to_string());

}
