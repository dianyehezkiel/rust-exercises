fn main() {
    println!("Hello, world!");

    let num = another_function(6);
    println!("{}", num * 3);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function(x: u8)-> u8 {
    println!("The value of x is: {x}");
    x
}