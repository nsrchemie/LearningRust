fn main() {
    let f = plus_one;
    let six = f(5);
    println!("{}",six);

    let a = [0,1,2,3];
    let part = &a[1..3];
    println!("Piece of array: {:?}. Length of full array is {}",part, a.len());

    let x: (i32, &str) = (1,"hello");
    println!("Tuple contains {:?}",x);

    let x1 = x.0;
    println!("First element of the above tuple is {}",x1);
}

fn plus_one(i: i32) -> i32 {
	i + 1
}