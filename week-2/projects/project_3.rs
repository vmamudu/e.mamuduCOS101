fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	let x:f64 = 1.0-(r/100.0); 
let x = f64::powf(x, n);
let a = x * p;
	println!("value of the TV after depreciation is {}", a);

}