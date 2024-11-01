
fn main() {
	let a = 1.2f64; let b = 1.0f64; let c = -1.0f64;
	let u = 2.6f64; let v = 2.0f64;
	println!("By f64: {}\t{}", a + c * b, u + c * v);
	
	let a = 1.2f32; let b = 1.0f32; let c = -1.0f32;
	let u = 2.6f32; let v = 2.0f32;
	println!("\tBy f32: {}\t{}", a + c * b, u + c * v);
}
