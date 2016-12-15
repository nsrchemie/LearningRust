fn main() {

	// let mut padovan = vec![1,1,1];
	// for i in 3..10 {
	// 	let next = padovan[i-3] + padovan[i-2];
	// 	padovan.push(next);
	// }
	// println!("P(1..10 = {:?}", padovan);

	// let alkaloids = vec!["Coniine".to_string(),
	//  "Nuciferine".to_string(),"Cycasin".to_string()];

	// // let alkaloids2 = alkaloids;
	// //The below will not work, 
	// //the second variable has ownership now of the original
	// // let alkaloids3 = alkaloids;

	// //We can clone the original data
	// let alkaloids2 = alkaloids.clone();
	// let alkaloids3 = alkaloids.clone();



	struct Person { name: String, 
					birth: i32 }
					
	let mut composers = Vec::new();
	composers.push(Person { name: "Palestrina".to_string(),birth: 1525 });

}
