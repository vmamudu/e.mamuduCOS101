/* rust program to read the height of a person
and then print if the perwson is tall, short,
or average height */

use std::io;

fn main() {

let mut input = String::new();

println!("\nEnter your height (in centimetres) :");
io::stdin().read_line(&mut input).expect("not a valid string :-(");
let height:f32 = input.trim().parse().expect("not a valid number !");

//else if ladder
if height >= 150.0 && height <= 170.0 {
println!("Your height is pretty average. ");
  } 
  else if height > 170.0 && height <= 195.0 {
    println!("You're tall. ");
  } 
  else if height < 150.0 && height >100.0 {
    println!("You are short. ");
  }
  else {
      println!("Abnormal height （〇_ｏ）");
  }
}
