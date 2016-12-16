//standard library has threads for running parallel code
use std::thread;

fn main() {
	//spawn method accepts a closure to execute in
	//a new thread
	//It returns a handle used to wait for the 
	//child thread to finish
	// let handle = thread::spawn(|| {
	// 	"This is inside a thread"
	// });

	// println!("{}",handle.join().unwrap());

	//Keep in mind that closures by default take variable 
	//references, this can cause an error since 
	//the thread could potentially outlive the references
	
	//use a move closure to force closure to take ownership
	//move closures move variables into them from the env.
	let x  = 1;
	let handle = thread::spawn(move || {
		println!("x is {}",x);
	});
	//extra code below for me to see inside thread
	//unwrap used to hide Ok/Err and show value
	//skipping use
	println!("{:?}",handle.join());
}
