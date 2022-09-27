use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");

    let number: u128 = number
        .trim()
        .parse()
        .expect("The number must be equal or greater than zero");

    let fib = fibonacci(number);

    println!("The fibonacci number is: {fib}");
}

fn fibonacci(x: u128) -> u128 {
    if x == 0 {
        return 0;
    }
    if x == 1 {
        return 1;
    }

    return x * fibonacci(x - 1);
}
