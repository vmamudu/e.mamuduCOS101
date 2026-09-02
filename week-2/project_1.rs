fn main () {
	let p:i64 = 520_000_000;

	let r:i64 = 10;

	let n:i64 = 5;

	let a = p*(1+(r/100))^n;

	println!("the amount is {}",a);

	let ci = a - p;

	println!("the compound interest is {}", ci);

}