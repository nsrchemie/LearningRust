fn main() {

	//working with structs, part 2
	struct Alien {
		health: u32,
		damage: u32
	}

	let mut bork = Alien{health: 100, damage: 5};
	println!("{}",bork.health);
}
