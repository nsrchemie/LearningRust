fn main() {

	//VECTORS
	//Vectors are dynamic arrays, typically implemented as Vec<T>
	//vectors always allocate on heap
	let v = vec![1,2,3,4,5];
	let slice = &v[0..3];
	println!("{:?}",slice);
}
