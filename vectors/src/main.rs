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



	//INDEXING
	// 	let v = vec![1,2,3];

	// 	//Note that indexing is not with i32 but with type usize
	// 	let i: usize = 0;
	// 	// let j: i32 = 0; //THIS WONT WORK
	// 	println!("The first element of v is {}",v[i]);


	//WORKING WITH OUT OF BOUNDS INDEXING ERRORS
	// let v = vec![1,2,3];
	// match v.get(7) {
	// 	Some(x) => println!("Item 7 is {}",x),
	// 	None => println!("The index doesnt exist"),
	// }

	//VECTOR ITERATION
	let mut v = vec![1,2,3];

	for i in &v {
		println!("reference to {}",i);
	}

	for i in &mut v {
		println!("mutable reference to {}", i);
	}

	for i in v {
		println!("Ownership of vector and element {}", i);
	}
	}

