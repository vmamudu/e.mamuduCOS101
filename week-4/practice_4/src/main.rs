//rust program to determine age pass

use std::io;

fn main() {

let mut input1 = String::new();
let mut input2 = String::new();

//name
println!("Hey! what's your name ? : ");
io::stdin().read_line(&mut input1).expect("not a valid string :-(");

//age
println!("How old are you? : ");
io::stdin().read_line(&mut input2).expect("not a valid string :-(");
let age:u8 = input2.trim().parse().expect("not a valid number !");

if age > 18 {
    println!("Welcome to the party {}!", input1);
} else {
    println!("Sorry, you're not old enough for this party {}:(", input1);
  }
}
