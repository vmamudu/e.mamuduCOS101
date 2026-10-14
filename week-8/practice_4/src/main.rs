fn main() {


//name vector
let name = vec!["Mofe", "Chudi", "Emeka", "Oyinbo", "Abdul", "amarachi", "Kene", "Ope"];

//age vector
let age = vec![16,17,19,22,20,21,18,16];

print!("\nAge allocation:\n");

//loop to iterate elements in vector
for i in 0..age.len()
    {
        //iterating through i on the vector
    print!("{} is {} years old\n", name[i],age[i]);
    }

}
