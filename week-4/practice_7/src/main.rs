//rust program to loop 
use std::io;

fn main() {

    println!("Enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
let mut num:i32 = input.trim().parse().expect("failed to read input");
    //while
    while num < 10 {
        println!("inside loop number value is {}",num );
        num+=1;
    }
    println!("outside loop number value is {}",num );
  
}
