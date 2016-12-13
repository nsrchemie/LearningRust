fn main() {

// Consumer, returns values with explicit type declaration for Vector
// let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
// Alternatively, can tell compiler to infer type of Vec
let one_to_one_hundred = (1..101).collect::<Vec<_>>();
let slice = &one_to_one_hundred[30..70];
println!("{:?}",slice);
}
