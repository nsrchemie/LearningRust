//standard library has threads for running parallel code
use std::thread;

fn main() {
	//spawn method accepts a closure to execute in
	//a new thread
	//It returns a handle used to wait for the 
	//child thread to finish
	thread::spawn(|| {
		println!("This is inside a thread");
	});

}
