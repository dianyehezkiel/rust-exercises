fn main() {
    const ORDER: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const LYRICS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        let o = ORDER[i];
        println!("On the {o} day of Christmas, my true love sent to me");
        for j in (0..=i).rev() {
            let l = LYRICS[j];
            println!("{l}");
            if j == 0 {
                println!("");
                break;
            };
        }
    }
}
