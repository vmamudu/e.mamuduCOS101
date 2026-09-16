// rust program to input name and age
use std::io;
fn main() {
    println!("\n~Student Information Managemnt System ! ✿~");
//input name
println!("\nPlease enter your name .");
let mut name = String::new();
io::stdin()
.read_line(&mut name)  
.expect("Couldn't read input :-(");
println!("Your name is {}", name );

//input age
println!("\nPlease enter your age .");
let mut age = String::new();
io::stdin()
.read_line(&mut age)
.expect("Couldn't read input :-(");
let age:u8 = age.trim()
.parse()
.expect("Input not an integer");
println!("You are {} year/s old", age);
}
