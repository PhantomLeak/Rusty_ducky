use std::io::{ Write };
mod greeting;
fn main() {
    greeting::ducky_greeting();
    let mut line = String::new();
    print!("What do you seem to be stuck on?: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("You said -> {}", line)
}
