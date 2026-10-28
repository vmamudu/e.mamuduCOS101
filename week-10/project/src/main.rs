//laptop struct
struct Laptop {
    brand: String,
    price: u32,
}
//price and quantity product
impl Laptop {
    fn totocost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}


fn main () {
    println!("Welcome to Alagba Electronics!");
    println!("\nProcessing order....
        \n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
//hp 
    let hp = Laptop{
        brand: String::from("HP"),
        price: 650_000,
    };
//ibm
 let ibm = Laptop{
        brand: String::from("IBM"),
        price: 755_000,
    };
//tobisha
 let tobisha = Laptop{
        brand: String::from("Toshiba"),
        price: 550_000,
    };
//dell
 let dell = Laptop{
        brand: String::from("Dell"),
        price: 850_000,
    };

let quantity:u32 = 3;

let cost_hp = hp.totocost(quantity);
let cost_ibm = ibm.totocost(quantity);
let cost_tobisha = tobisha.totocost(quantity);
let cost_dell = dell.totocost(quantity);

let total = cost_hp + cost_ibm + cost_dell + cost_tobisha;
println!("\nYoure buying 3 of each: {}, {}, {}, {}", hp.brand, dell.brand, tobisha.brand,ibm.brand );
println!("The total cost for buying {} units of each brand is {}", quantity, total );
println!("\nThank you! ✿ ᓚᘏᗢ
    \n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}
