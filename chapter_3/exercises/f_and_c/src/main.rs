use std::io;

fn main() {
    println!("Choose which converter to use:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut conv_choice = String::new();

    let mut yes_no = String::from("y");

    println!("Please enter 1 or 2");

    io::stdin()
        .read_line(&mut conv_choice)
        .expect("Failed to readline");

    println!("This is input value: {}", conv_choice);

    if conv_choice.trim() == "1" {
        while yes_no == "y" {
            let mut temperature = String::new();
            println!("Enter the temperature in Fahrenheit:");
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to readline");

            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };

            let conv_temp = (temperature-32.0)*5.0/9.0;

            println!("Temperature in Celsius is: {:.2} degree", conv_temp);
            println!("Do you want to convert again? (y/n)");
            io::stdin()
                .read_line(&mut yes_no)
                .expect("Failed to readline");
        }
    }

    println!("user type 2")
}
