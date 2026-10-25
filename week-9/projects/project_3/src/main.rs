use std::io::Write;


fn main() {
    
println!("EFCC file merge!");

    let header = vec!["Commissioner", "Ministry", "Zone"];

    let cumisner = vec![
    "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye"
    ];

let minsty = vec![
"Internal Affairs",
"Justice",
"Defense",
"Power & Steel",
"Petroleum",
    ];

    let zone = vec![
    "South West",
    "North West",
    "South South",
    "South West",
    "South East",
    ];

//file creation
    let mut file = std::fs::File::create("EFCC_Merge.csv").expect("create failed");

//header
    for i in 0..header.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(header[i].as_bytes()).expect("write failed");
        }
file.write_all("\n".as_bytes()).expect("write failed");


//row 1
file.write_all(cumisner[0].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(minsty[0].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(zone[0].as_bytes()).expect("failed to write");

file.write_all("\n".as_bytes()).expect("failed to write");

//row 2
file.write_all(cumisner[1].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(minsty[1].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(zone[1].as_bytes()).expect("failed to write");

file.write_all("\n".as_bytes()).expect("failed to write");

//row 3
file.write_all(cumisner[2].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(minsty[2].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(zone[2].as_bytes()).expect("failed to write");

file.write_all("\n".as_bytes()).expect("failed to write");

//row 4
file.write_all(cumisner[3].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(minsty[3].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(zone[3].as_bytes()).expect("failed to write");

file.write_all("\n".as_bytes()).expect("failed to write");

//row 5
file.write_all(cumisner[4].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(minsty[4].as_bytes()).expect("failed to write");
file.write_all(",".as_bytes()).expect("failed to write");
file.write_all(zone[4].as_bytes()).expect("failed to write");

file.write_all("\n".as_bytes()).expect("failed to write");

println!("done!");
}
