fn main() {
    //using vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of vec::new is {}", v.len());

    //using macro
    let v = vec!["Tobechi", "collins", "kristin", "tito", "ren"];

    //printing the size of vector
    println!("\n The length of vec macro is: {}", v.len());

}
