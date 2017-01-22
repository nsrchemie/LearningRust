fn main() {
fn sqroot(r: f32) -> Result<f32, String> {
if r < 0.0 {
return Err("Number cannot be negative!".to_string());
}
Ok((r).sqrt())
}

println!("{:?}", sqroot(-2.1));
}
