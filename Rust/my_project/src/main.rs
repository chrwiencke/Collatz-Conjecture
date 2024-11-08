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
    let number = 837799;
    collatz(number);
}