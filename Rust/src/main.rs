use std::env;

fn collatz(mut number: i64) {
    while number != 1 {
        if number % 2 != 0 {
            number = number * 3 + 1;
            println!("{}", number);
        } else {
            number = number / 2;
            println!("{}", number);
        } 
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./main <number>");
        std::process::exit(1);
    }

    let number: i64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            std::process::exit(1);
        }
    };

    collatz(number);
}