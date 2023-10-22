use std::io::Write;
use rand::Rng;

const RESPONSE_OPTIONS: [&'static str; 3] = ["Okay...Then what?", "I see...", "Quack!"];

pub fn rubber_ducky() {
    let mut line = String::new();
    let mut debugging_complete: bool = false;
    print!("What do you seem to be stuck on?: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    print!("Okay...walk me through what the code is currently doing, line by line: ");
    while !debugging_complete {
        let mut line = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        debugging_complete = write_response(line);
    }
}


fn write_response(response: String) -> bool{
    if !response.contains("got it") &&!response.contains("figured it out") && !response.contains("thank you") {
        let index = rand::thread_rng().gen_range(0..3);
        print!("{} :", RESPONSE_OPTIONS[index]);
        return false;
    } else {
        print!("*Quack* (I'm happt to help, come again if you need anything)");
        return true;
    }
}