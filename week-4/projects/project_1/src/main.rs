//quadratic roots
use std::io;


fn main() {
    println!("Root of quadratic equation Calculator !
        \nᓚᘏᗢ
        \n(hint: Ax² +Bx + C = 0)");
    println!("Input A");
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("couldn't read input :-(");
let a:f32 = input1.trim().parse().expect("not a valid number !");

println!("Input B");
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("couldn't read input :-(");
let b:f32 = input2.trim().parse().expect("not a valid number !");

println!("Input C");
let mut input3 = String::new();
io::stdin().read_line(&mut input3).expect("couldn't read input :-(");
let c:f32 = input3.trim().parse().expect("not a valid number !");

let d = b*b - 4.0*a*c; 
let x1 = -b + d.sqrt()/2.0*a;
let x2 = -b - d.sqrt()/2.0*a; 
if d > 0.0 {
    println!("roots of equation = {}, {}", x1, x2);
}
else if d == 0.0 {
   println!("root of equation = {} (repeated roots!)", x1);  
}
else {
     println!("!!! there are no real roots of this equation");
 }
}
