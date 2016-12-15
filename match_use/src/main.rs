fn main() {

	//match is used for exhaustive conditionals

	// let x = 5;

	// match x {
	// 	1 => println!("One"),
	// 	2 => println!("Two"),
	// 	_ => println!("Value above 2"),
	// }

	//match can also be used in a let binding 
	//to convert between types 

	// let number = match x {
	// 	1 => "one",
	// 	2 => "two",
	// 	_=> "Other number",
	// };

	// println!("{}",number);

	//MATCHING ON ENUMS

// 	enum Message {
// 		Quit,
// 		ChangeColor(i32,i32,i32),
// 		Move {x: i32, y:i32},
// 		Write(String),
// 	}

// fn quit() {/* ... */}
// fn change_color(r: i32,g: i32, b: i32){/* ... */}
// fn move_cursor(x:i32, y:i32){/*...*/}

// fn process_message(msg: Message) {
// 	match msg {
// 		Message::Quit => quit(),
// 		Message::ChangeColor(r,g,b) => change_color(r,g,b),
// 		Message::Move {x,y} => move_cursor(x,y),
// 		Message::Write(s) => println!("{}",s),
// 	};
// }

	//MATCHING MULTIPLE PATTERNS
	//using | for OR conditional 
	let x = 2;

	match x {
		1 | 2 => println!("One or two"),
		_ => println!("Something else"),
	}
}
