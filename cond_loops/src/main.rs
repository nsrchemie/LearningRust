fn main() {
	// let x = 5;

	// let y = if x == 5 {
	// 	10
	// }
	// else {
	// 	15
	// };

	// Rewrite

	// let x = 5;
	// let y = if x==5 {10} else {15};

	// println!("y is equal to {}",y);



	//loop can be used for situations like 'while true {}'

	//loop { println!("This is an infinite loop!")};

	//while loops
	let mut x = 5;
	let mut done = false;

	while !done {
		x += x -3;

		println!("{}",x);

		if x % 5 == 0 {
			done = true;
		}
	}
}
