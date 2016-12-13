// extern crate carboxyl;
// use carboxyl::{Sink, Stream, Signal};

// mutable borrows
	// let mut x = 5;
	// {
	// 	let y = &mut x;
	// 	*y += 1;
	// }
 //    println!("{}",x);

 // Streams/signals

// let sink = Sink::new();
// let stream = sink.stream();
// let signal = stream.hold(3);

// assert_eq!(signal.sample(), 3);

// sink.send(5);
// assert_eq!(signal.sample(), 5);
// let x :i32 = 2;
// let mut y :i32 = 3;
// y = x;
// println!("{}",x);
use std::io::Write;
use std::str::FromStr;

fn main() {

	let mut numbers = Vec::new();

	for arg in std::env::args().skip(1) {
		numbers.push(u64::from_str(&arg)
				.expect("error parsing argument"));
	}


	if numbers.len() == 0 {
		writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
		std::process::exit(1);
	}


	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d, *m);
	}

	println!("Greatest Common Divisor of {:?} is {}",numbers, d);


fn gcd(mut n: u64, mut m: u64) -> u64 {
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t =m; m=n;n=t;
	}
	m = m % n;
	}
	n	
}


// println!("{}",gcd(64, 42));
// #[test]
// fn test_gcd() {
// 	assert_eq!(gcd(2*5*11*17,3*7*13*19),1);

// 	assert_eq!(gcd(2*3*5*11*17,3*7*11*13*19),3*11);
}


