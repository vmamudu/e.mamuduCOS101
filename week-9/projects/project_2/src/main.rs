use::std::io::Write;

fn main() {


//header
    let header = vec! ["Student Name", "Matric Number", "Department", "Level"];

//student rows and columns
let rows_colums = vec![
("Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"),
("ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"),
("Accounting", "Economics", "Computer", "Electrical", "Mechanical"),
("300", "100", "200", "200", "100"),
];


//creating the file
let mut file = std::fs::File::create("pau_smis.csv").expect("create failed");
file.write_all("PAU's Student Management Information System Records"
    .as_bytes()).expect("write failed");    
//space
file.write_all("\n".as_bytes()).expect("write failed");

//doing the header
for i in 0..header.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(header[i].as_bytes()).expect("write failed");
        }
file.write_all("\n".as_bytes()).expect("write failed");

//doing the body
for i in 0..rows_colums.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(rows_colums[i].0.as_bytes()).expect("write failed");
        }


file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..rows_colums.len(){

    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(rows_colums[i].1.as_bytes()).expect("write failed");
        }

file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..rows_colums.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(rows_colums[i].2.as_bytes()).expect("write failed");
        }

file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..rows_colums.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(rows_colums[i].3.as_bytes()).expect("write failed");
        }

file.write_all("\n".as_bytes()).expect("write failed");

for i in 0..rows_colums.len(){
    if i > 0 {file.write_all(",".as_bytes()).expect("write failed");}
    file.write_all(rows_colums[i].4.as_bytes()).expect("write failed");
        }
}
