fn main() {

//borrowing

// fn sum_vec(v: &Vec<i32>) -> i32 {
// 	//create an iterator from the referenced vector, then consume it by adding
// 	//the elements together [a,b,c] -> a+b+c
// 	return v.iter().fold(0, |a, &b| a + b);
// }

// fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
// 	//take two referenced vectors containing 32 bit integers
// 	let s1 = sum_vec(v1);
// 	let s2 = sum_vec(v2);
// 	//add the totals and leave off semicolon to return for printing
// 	s1 + s2
// }

// 	let v1 = vec![1,2,3];
// 	let v2 = vec![4,5,6];

// 	let answer = foo(&v1, &v2);
// 	println!("{}",answer);


//mut referencing/borrowing, one at a time
// let mut x = 5;
// {
// let y = &mut x; 
// *y += 1;
// //the extra curly braces compartmentalize the mutable borrow and we can 
// //work with the immutable borrow x for printing
// //Only one borrow/reference is allowed at a time
// //this function goes:
// //x -> y -> x
// }
// println!("{}", x);





}
