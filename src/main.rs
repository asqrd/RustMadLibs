use std::io::{stdin,stdout,Write};

fn main() {
    let name = get_input("Please enter your name: ");
    let subject = get_input("Please enter a subject: ");
    let verb = get_input("Please enter a verb (past tense): ");
    let object = get_input("Please enter an object: ");

    println!("{name} {verb} the {subject} with their {object}",
             name=name,
             verb=verb,
             subject=subject,
             object=object);
}

/*
 * Takes a static str `msg`
 * Returns input from user console.
**/
fn get_input(msg: &'static str) -> String {
    let mut input = String::new();

    print!("{}", msg); // Print message.

    let _=stdout().flush(); // Flust stdout.

    stdin().read_line(&mut input).expect("Error reading input");

    input = clean_input(input);

    input // Return cleaned input.
}

fn clean_input(mut input: String) -> String {
   // Remove newline.
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    // Remove carriage return.
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }
    input // Return cleaned input.
}
