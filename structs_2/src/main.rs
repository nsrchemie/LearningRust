fn main() {

	//working with structs, part 2
	struct Alien {
		health: u32,
		damage: u32
	}
	// let mut bork = Alien{health: 100, damage: 5};
	// println!("{}",bork.health);

	//Let's say the health parameter cannot exceed 100
	//we can make this limitation in a method
	impl Alien {
		fn new(mut h: u32, d: u32) ->Alien {
		if h > 100 {h = 100; }
		Alien {health:h, damage:d}
	}
}
	//Now we have permanently set an upper limit for health
	//and also simplified the initializing command
	let mut bork = Alien::new(150,15);
	println!("{}",bork.health);
}
