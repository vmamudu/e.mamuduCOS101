fn main() {
let empty_string = String::new();
println!("Length of empty_string is {}", empty_string.len() ); //length should be 0 because its an empty string

let content_string = String::from("ComputerScience");
println!("Length of content_string is {}", content_string.len() ); //length wont be 0 this time

}
