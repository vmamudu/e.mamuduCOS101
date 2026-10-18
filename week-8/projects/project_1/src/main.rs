//* THE APS LEVEL CHECKER*//
use std::io;

fn ask(question: &str) -> String {
    println!("{}", question);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    answer.trim().to_string()
}


fn off() {
    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    print!("Office admin categories include; \n{:?}", admin);
    println!("\nWhich category do you belong to?");
    let mut off_cat = String::new();
    io::stdin().read_line(&mut off_cat).expect("failed to read input :(");

    let office = match off_cat.trim().to_lowercase().as_str() {
        "intern" => 0,
        "administrator" => 1,
        "senior administrator" => 2,
        "office manager"  => 3,
        "director"  => 4,
        "ceo" => 5,
        _ => 6,
    };
let officelevel = levels[office];
println!("Your APS level is {} ", officelevel);
}

fn academic() {
    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let acad = vec![" ","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    print!("Academic categories include; \n{:?}", acad);
    println!("\nWhich category do you belong to?");
    let mut acad_cat = String::new();
    io::stdin().read_line(&mut acad_cat).expect("failed to read input :(");

    let demic = match acad_cat.trim().to_lowercase().as_str(){
        " " => 0,
        "research assistant" => 1,
        "phd candidate" => 2,
        "post-doc researcher" => 3,
        "senior lecturer" => 4,
        "dean" => 5,
        _ => 6,
    };
let acadlevel = levels[demic];
println!("Your APS level is {} ", acadlevel);
}

//lawyer
fn law() {
    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let lawlaw = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    print!("lawyer categories include; \n{:?}", lawlaw);
    println!("\nWhich category do you belong to?");
    let mut law_cat = String::new();
    io::stdin().read_line(&mut law_cat).expect("failed to read input :(");

    let lawyer = match law_cat.trim().to_lowercase().as_str(){
        "paralegal" => 0,
        "junior associate" => 1,
        "associate" => 2,
        "senior associate 1-2" => 3,
        "senior associate 3-4" => 4,
        "partner" => 5,
        _ => 6,
    };
let lawlevel = levels[lawyer];
println!("Your APS level is {} ", lawlevel);
}


//teacher
fn teach() {
    let levels = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];
    let teach = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    print!("teacher categories include; \n{:?}", teach);
    println!("\nWhich category do you belong to?");
    let mut teach_cat = String::new();
    io::stdin().read_line(&mut teach_cat).expect("failed to read input :(");

    let teacher = match teach_cat.trim().to_lowercase().as_str(){
        "placement" => 0,
        "classroom teacher" => 1,
        "senior teacher" => 2,
        "leading teacher" => 3,
        "deputy principal" => 4,
        "principal" => 5,
        _ => 6,
    };
let teachlevel = levels[teacher];
println!("Your APS level is {} ", teachlevel);
}


fn main() {
println!("Welcome to the APS Level Checker! ");
println!("\nWhat's your name?");
let mut name = String::new(); io::stdin().read_line(&mut name).expect("invalid!");
println!("Welcome {} !\n Time to select a role. ",name );
    let role = ask("\nChoose a role:
        \nChoose from;
        \nOffice administrator
        \nAcademic
        \nLawyer
        \nTeacher");

let role = match role.trim().to_lowercase().as_str(){
    "office administrator" => off(),
    "academic" => academic(),
    "lawyer" => law(),
    "teacher" => teach(),
    _ => println!("invalid selection!"),
};

}


