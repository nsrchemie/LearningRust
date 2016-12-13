fn main() {

// Consumer, returns values with explicit type declaration for Vector
// let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
// Alternatively, can tell compiler to infer type of Vec
// let one_to_one_hundred = (1..101).collect::<Vec<_>>();
// let slice = &one_to_one_hundred[30..70];


//Find Consumer
// let over_twenty = (0..80).find(|x| *x > 20);

// //Without the below if we just printed the variable over_twenty
// //We would receive an output of 'Some(21)', showing the first match
// match over_twenty {
// 	Some(_) => println!("Met conditions"),
// 	None => println!("Conditions not met"),
// }


//Fold Consumer
//format for fold is fold(base, accumulator, element)
// let sum = (2..5).fold(0, |sum, x| sum + x);
//First Iter: sum = 0, x first element of range (2)
//sum + x = 2
//Second Iter: sum = 2, x = 3
//sum + x = 5
//Third Iter: sum = 5, x = 4
//sum + x = 9
// //9 is the final value
// println!("{:?}",sum);


// let nums = (1..100).collect::<Vec<i32>>();
//The first part does not generate the actual numbers 
//but creates a value that represents the range
//The consumer tells the range to generate and send it values

//The other basic iterator aside from the range is iter()
//iter() can convert a vector into a simple iterator
// let nums = vec![1,2,3];

// for num in nums.iter() {
// 	println!("{}",num);
// }


//Iterator Adaptors

//Map
// let m = (1..100).map(|x| x + 1);
// println!("{:?}",m);

// //Take
// for i in (1..).take(5) {
// 	println!("{}",i);
// }

//Filter
//boolean conditional adapter
// for i in (1..100).filter(|&x| x % 2 == 0) {
// 	println!("{}",i);
// }

let chained = (1..)
	.filter(|&x| x % 2 == 0)
	.filter(|&x| x % 3 == 0)
	.take(5)
	.collect::<Vec<i32>>();
println!("{:?}",chained);
}
