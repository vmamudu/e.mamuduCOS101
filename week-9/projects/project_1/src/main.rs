use std::io::Write;


fn main() {

let nb1 = "Lager, Stout, Non-alcoholic";
let nb2 = "\n33 Export, Legend, Maltina";
let nb3 = "\nDesperados, Turbo King, Amstel Malta";
let nb4 = "\nGoldberg, Williams, Malta Gold";
let nb5 = "\nGulder,,Fayrouz";
let nb6 = "\nHeineken, , ,";
let nb7 = "\nStar,,,";


let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("create failed");
file.write_all("Nigerian Breweries PLC\n Categories of Drinks"
    .as_bytes()).expect("write failed");
file.write_all(nb1.as_bytes()).expect("write failed");
file.write_all(nb2.as_bytes()).expect("write failed");
file.write_all(nb3.as_bytes()).expect("write failed");
file.write_all(nb4.as_bytes()).expect("write failed");
file.write_all(nb5.as_bytes()).expect("write failed");
file.write_all(nb6.as_bytes()).expect("write failed");
file.write_all(nb7.as_bytes()).expect("write failed");

println!("\nData written to file.");


}
