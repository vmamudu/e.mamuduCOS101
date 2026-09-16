// rust program to calculate the area of a trianlge
use std::io;

fn main() {

    println!("area of triangle calculator <3 !");

let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

//first side
println!("\nInput the first side of triangle: ");
io::stdin().read_line(&mut input1).expect("not a valid string :-(");
let a:f32 = input1.trim().parse().expect("not a valid number !");


//second side
println!("\nInput the second side of triangle: ");
io::stdin().read_line(&mut input2).expect("not a valid steing :-(");
let b:f32 = input2.trim().parse().expect("not a valid number !");

//third side
println!("\nInput the third side of triangle: ");
io::stdin().read_line(&mut input3).expect("not a valid string :-(");
let c:f32 = input3.trim().parse().expect("not a valid number !");

//area
let s:f32 = (a + b + c) / 2.0;
let mut area:f32 = s * (s - a) * (s - b) * (s - c);
area = area.sqrt();


println!("Area of a triangle : {}", area);
}