fn main() {

	//VECTORS
	//Vectors are dynamic arrays, typically implemented as Vec<T>
	//vectors always allocate on heap
	// let v = vec![1,2,3,4,5];
	// let slice = &v[0..3];
	// println!("{:?}",slice);

	//to create a vector of duplicates
	// 	let v = vec![0; 10];
	// 	println!("{:?}",v);



	//indexing
	let v = vec![1,2,3];

	//Note that indexing is not with i32 but with type usize
	let i: usize = 0;
	// let j: i32 = 0; //THIS WONT WORK
	println!("The first element of v is {}",v[i]);
}
