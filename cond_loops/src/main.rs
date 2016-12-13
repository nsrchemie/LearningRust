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

	//WHILE LOOPS
	// let mut x = 5;
	// let mut done = false;
	// while !done {
	// 	x += x -3;
	// 	println!("{}",x);
	// 	if x % 5 == 0 {
	// 		done = true;}}

	//for loop with range (end number of ranges are exclusive)
	// for x in 0..10 {
	// 	println!("{}",x);
	// }

	//enumerate helps keep track of how many times you have already looped
	//enumerate on a range
	// for (index, value) in (5..10).enumerate() {
	// 	println!("Index = {} and value = {}",index, value);
	// }

	//enumerate on iterators
	// let lines = "hello\nworld".lines();

	// for (line_number, line) in lines.enumerate() {
	// 	println!("{}: {}", line_number, line);
	// }







	//DISRUPTING ITERATION/ENDING ITERATION EARLY
	//USING BREAK
	//this is a slight rewrite of the while example starting on line 23
	// let mut x = 5;

	// loop {
	// 	x += x - 3;

	// 	println!("{}",x);

	// 	if x % 5 == 0 {break;}
	// }

	//USING CONTINUE
	//this prints odd numbers
	// for x in 0..10 {
	// 	if x % 2 == 0 {continue;}
	// 	println!("{}",x);
	// }

	//LOOP LABELS

	'outer: for x in 0..10 {
		'inner: for y in 1..10 {
			if x % 2 == 0 { continue 'outer;}
			if y % 2 == 0 { continue 'inner;}
			println!("x: {}, y: {}",x, y);
		}
	}



}
