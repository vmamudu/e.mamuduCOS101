//rust program to count numbers
use std::io;

//input lower bound
fn main() {
println!("Enter lower bound");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Failed to read input");
let lower_bound:i32 = input1.trim().parse().expect("failed to read input");

//input upper bound
println!("Enter upper Bound");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("Failed to read input");
let upper_bound:i32 = input2.trim().parse().expect("Failed to read input");

//loop
for x in lower_bound..upper_bound{ //doesnt include upper bound
println!("Count level is {}",x );
  }
}