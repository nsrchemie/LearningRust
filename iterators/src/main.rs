fn main() {

// Consumer, returns values
let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
let slice = &one_to_one_hundred[30..70];
println!("{:?}",slice);
}
