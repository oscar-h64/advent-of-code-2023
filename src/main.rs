mod day1;

use std::{env, fs};

fn run_day(day: u8, puzzle: &str, f: fn(&str) -> u64) {
    println!("Running day {} puzzle {}\n", day, puzzle);

    let file_name = format!("inputs/{}.txt", day);

    let inp_owned = fs::read_to_string(file_name)
        .expect("Failed to read the input file");
    let inp = inp_owned.as_str();

    println!("Input read, running code...\n");

    let res = f(inp);

    println!("Output: {}", res);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("You should provide 2 arguments, not {}", args.len());
        return
    }

    let day: u8 = args[1].parse().expect("Expected 1st argument to be a number");
    let puzzle = args[2].as_str();

    match (day, puzzle) {
        (1, "A") => {
            run_day(1, "A", day1::a);
        }
        (1, "B") => {
            run_day(1, "B", day1::b);
        }
        (_, _) => {
            println!("Unknown day/puzzle {} {}", day, puzzle)
        }
    }
}
