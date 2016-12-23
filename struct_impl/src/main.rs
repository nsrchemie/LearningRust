fn main() {

	//Note that structures are compound data types
	//We can even define structures with generic types

	struct Pair<T> {
		first: T,
		second: T,
	}

	//Using the above we can now create different structure instances
	let lucky_number_pair: Pair<i32> = Pair { first: 13, second: 7};
	println!(" My first lucky number is {:?}", lucky_number_pair.first);

	let tobacco_alkaloid_pair: Pair<&str> = Pair { first: "Nicotine", second: "Anabasine"};
	println!("Everyone talks about the {:?} in Tobacco but the FAR more interesting drug in it is {:?}",tobacco_alkaloid_pair.first, tobacco_alkaloid_pair.second );
}
