fn main() {
    
    fn gcd(mut n: u64, mut m: u64) -> u64 {
    	assert!(n != 0 && m != 0);
    	let nums = [None,];
    	while m != 0 {
    		if m < n {
    			let temp = m; m = n; n = temp;
    		}
    		m = m % n;
    	}
    	n
    }
    println!("GCD results for 42, 88 {}",gcd(42,88));
}
