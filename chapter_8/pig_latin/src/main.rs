use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("Please input any word(s).");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = match input.trim().parse() {
            Ok(text) => text,
            Err(_) => continue,
        };

        break;
    }

    let result = parse_pig_latin(&input);

    println!("{}", result);
}

fn parse_pig_latin(text: &str) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        let mut temp_word = String::new();
        let mut ay = String::from("hay");
        for (i, char) in word.char_indices() {
            if i == 0 && !is_vowel(&char) {
                ay = String::from(format!("{}ay", &char));
                continue;
            }
            temp_word.push(char);
        }

        result.push_str(&format!("{}-{} ", &temp_word, &ay));
    }

    result = String::from(result.trim());

    return result;
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'i' | 'e' | 'u' | 'o' => return true,
        _ => return false,
    }
}
