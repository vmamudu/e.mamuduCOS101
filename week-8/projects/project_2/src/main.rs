use std::io;


fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer.trim().to_string()
}

fn main() {

    let mut applicants: Vec<(String, u8)> = Vec::new();

    println!("Welcome to the EY Nigeria interview desk!");
    println!("How many candidates are we seeing today?");
    let total: u8 = read_input().parse().expect("Please enter a valid number");

    if total == 0 {
        println!("No candidates today. Enjoy the quiet day!");
        return;
    }

    for number in 1..=total {
        println!("\nCandidate #{}, what is your name?", number);
        let name = read_input();

        println!("Nice to meet you, {}. How many years have you been programming?", name);
        let years: u8 = read_input().parse().expect("Please enter a valid number");

        applicants.push((name, years));
    }


    let mut top = 0;
    for i in 1..applicants.len() {
        if applicants[i].1 > applicants[top].1 {
            top = i;
        }
    }
    let top_years = applicants[top].1;


    let mut winners: Vec<&String> = Vec::new();
    for (name, years) in &applicants {
        if *years == top_years {
            winners.push(name);
        }
    }

    println!("\nInterview Summary");
    for (name, years) in &applicants {
        println!("{} - {} years", name, years);
    }

    if winners.len() == 1 {
        println!(
            "\n{} has the most experience with {} years of programming. Best fit for the job!",
            winners[0], top_years
        );
    } else {
        println!(
            "\nIt's a tie! {:?} each have {} years of programming experience !!!",
            winners, top_years
        );
    }
}