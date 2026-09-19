use std::io;

fn main() {
 println!("Welcome to the Incentive Calculator™");
//experience input
 println!("\nPLease input YEARS of experience .");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("not a valid input.");
let exp:u8 = input1.trim().parse().expect("not a valid number !");

if exp <= 2 { //0-2 years : inexperienced
    println!("Your annual incentive is N100,000");}
    else { //>2 years : experienced
//age input
println!("Please input your age .");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("not a valid input.");
let age:u8 = input2.trim().parse().expect("not a valid number !");

if age < 30 {  //young
    println!("your annual incentive is N1,300,000");
  } 
  else if age >=30 && age < 40 { //mid age
    println!("Your annual incentive is N1,480,000");
   }
 else if age >=40 { //older
    println!("Your annual incentive is N1,560,000");
 }

 }
}
