mod day1;
mod day2;
mod day3;
mod day4;

use std::{env, fs};
use std::time::Instant;

fn run_day(day: u8, puzzle: &str, f: fn(&str) -> u64) {
    println!("Running day {} puzzle {}\n", day, puzzle);

    let file_name = format!("inputs/{}.txt", day);

    let inp_owned = fs::read_to_string(file_name)
        .expect("Failed to read the input file");
    let inp = inp_owned.as_str().trim_end();

    println!("Input read, running code...\n");

    let now = Instant::now();
    let res = f(inp);
    let puzzle_time = now.elapsed();

    println!("Output: {}", res);
    println!("Solution took {:?}", puzzle_time);
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
        (1, "A") => run_day(1, "A", day1::a),
        (1, "B") => run_day(1, "B", day1::b),
        (2, "A") => run_day(2, "A", day2::a),
        (2, "B") => run_day(2, "B", day2::b),
        (3, "A") => run_day(3, "A", day3::a),
        (3, "B") => run_day(3, "B", day3::b),
        (4, "A") => run_day(4, "A", day4::a),
        (4, "B") => run_day(4, "B", day4::b),
        (_, _) => println!("Unknown day/puzzle {} {}", day, puzzle),
    }
}
