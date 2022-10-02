use std::collections::HashMap;
use std::io;

fn main() {
    let mut nums: Vec<i32> = Vec::new();

    'input_loop: loop {
        println!("Input all numbers divided by space.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        for inp in input.split_whitespace() {
            match inp.trim().parse() {
                Ok(num) => nums.push(num),
                Err(_) => {
                    println!("Please only input number.\n");
                    nums.clear();
                    continue 'input_loop;
                }
            };
        }

        break;
    }

    let median = find_median(&nums);
    let modus = find_modus(&nums);

    print!("The numbers is: {:?}\n", nums);

    match median {
        Some(n) => println!("Median is: {:?}", n),
        None => println!("Cannot find median, there is no number in the current list"),
    }

    match modus {
        Some(n) => println!("Modus is: {}", n),
        None => println!("Cannot find modus, there is no number in the current list"),
    }
}

fn find_median(nums: &Vec<i32>) -> Option<f64> {
    let len = nums.len();
    if len == 0 {
        return None;
    }

    let med_index = len / 2;

    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    let sum = sorted_nums[med_index - (1 - len % 2)] + sorted_nums[med_index];
    return Some(f64::from(sum) / 2.0);
}

fn find_modus(nums: &Vec<i32>) -> Option<i32> {
    let mut num_count = HashMap::new();

    for i in nums {
        let entry = num_count.entry(*i).or_insert(0);
        *entry += 1;
    }

    let modus = num_count
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| *k);

    return modus;
}
