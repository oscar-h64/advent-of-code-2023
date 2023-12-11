use std::collections::HashMap;
use std::ops::Add;

// check_valid returns whether this character, the above it (if exists) or the one below it (if exists)
// is a symbol in the map. It also resets a list of co-ordinates for which ones are symbols
fn check_valid(i: &usize, j: &usize, symbol_map: &Vec<Vec<bool>>) -> (bool, Vec<(usize,usize)>) {

    // Check this cell, the one above (if it exists) and the one below to see if they contain a symbol
    let mut to_check = vec![(i.clone(), j.clone()), (i.clone()+1, j.clone())];
    if *i > 0 {
        to_check.push((i.clone()-1, j.clone()));
    }

    let valid: Vec<(usize,usize)> = to_check
        .iter()
        .filter_map(|(k, l)| {
            let to_keep = symbol_map
                .get(k.clone()) // get row if it exists
                .map(|r| r.get(l.clone()).unwrap()) // get column, which should exist
                .unwrap_or(&false) // if the row doesn't exist, default to false
                .clone();

            if to_keep {
                Some((k.clone(), l.clone()))
            } else {
                None
            }
        })
        .collect();

    (valid.len() > 0, valid)
}

// process_input split the input into lines and generates the symbol map
fn process_input(inp: &str, symbol_map_fn: fn(char) -> bool) -> (Vec<String>, Vec<Vec<bool>>) {
    let lines: Vec<String> = inp
        .split_whitespace() // Split into lines
        .map(|s| s.to_owned().add(".")) // BODGE: add a trailing . to handle numbers at the end of lines
        .collect();

    // 2D vector of where there is a symbol (dots don't count)
    let symbol_map: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| line.chars().map(symbol_map_fn).collect()) // Turn line into a vector of bools
        .collect();

    (lines, symbol_map)
}

pub fn a(inp: &str) -> u64 {

    // Build the symbol map of anything that isn't a dot or number
    let (lines, symbol_map) = process_input(inp, |c| !c.is_ascii_digit() && c != '.');

    lines.iter().zip(0..).map(|(line, i)| {

        // We read the line char by char, building up a number when we encounter symbol chars. We then
        // parse a number once we reach the end of a set of consecutive symbols
        let mut current_num: Option<String> = None;
        let mut current_num_valid = false;
        let mut sum = 0;

        for (c, j) in line.chars().zip(0..) {

            if c.is_ascii_digit() {
                // Either add this digit to the existing number being built, or if there is no number in progress start with this digit
                current_num = match current_num {
                    None => Some(c.into()),
                    Some(s) => Some(s.add(c.to_string().as_str()))
                };

                current_num_valid |= check_valid(&i, &j, &symbol_map).0;


            } else {

                let this_char_valid = check_valid(&i, &j, &symbol_map).0;

                // Handle the case of just finishing a number
                if let Some(num) = current_num {
                    // If it was valid (either already, or based on this column), add it to the sum
                    if current_num_valid || this_char_valid {
                        sum += num.clone().parse::<u64>().expect("Expected to parse number");
                    }

                    // Reset the string accumulator
                    current_num = None;
                };

                // Set this based on whether the current column is a symbol (or has one above/below)
                // This is because we might be about to start a number
                current_num_valid = this_char_valid.clone();

            }
        }

        sum
    }).sum()
}

pub fn b(inp: &str) -> u64 {

    // Build a symbol map that only cares about *
    let (lines, symbol_map) = process_input(inp, |c| c == '*');

    // This is keyed by * location, and contains a list of adjacent numbers. By producing this at the
    // end we can calculate which are gears, and sum their ratios
    let mut star_map: HashMap<(usize,usize),Vec<u64>> = HashMap::new();

    for (line, i) in lines.iter().zip(0..) {

        let mut current_num: Option<String> = None;
        let mut current_num_valid = false;

        // We build a list of star locations around a number
        let mut star_locations: Vec<(usize,usize)> = vec![];

        for (c, j) in line.chars().zip(0..) {

            if c.is_ascii_digit() {
                // Either add this digit to the existing number being built, or if there is no number in progress start with this digit
                current_num = match current_num {
                    None => Some(c.into()),
                    Some(s) => Some(s.add(c.to_string().as_str()))
                };

                // Add any stars above or below this character to the list
                let (is_this_col_valid, mut locs) = check_valid(&i, &j, &symbol_map);
                current_num_valid |= is_this_col_valid;
                star_locations.append(&mut locs);


            } else {

                let (this_col_valid, locs) = check_valid(&i, &j, &symbol_map);

                // Handle the case of just finishing a number
                if let Some(num) = current_num {
                    // If it was valid (either already, or based on this column), add it to the list for any relevant stars
                    if current_num_valid || this_col_valid {
                        let this_num = num.clone().parse::<u64>().expect("Expected to parse number");

                        // Add this number to any relevant star locations. To account for any relevant
                        // stars in the column after the number ends, we add `locs` to the list here.
                        // WE CLONE `locs` BECAUSE `append` CONSUMES THE VALUES AND LEAVES `locs` EMPTY
                        star_locations.append(&mut locs.clone());
                        for loc in star_locations {
                            star_map.entry(loc)
                                .and_modify(|nums| nums.push(this_num.clone()))
                                .or_insert(vec![this_num]);
                        }
                    }

                    // Reset the string accumulator
                    current_num = None;
                };

                // Reset the values based on this column, as we might be able to start a number
                current_num_valid = this_col_valid.clone();
                star_locations = locs;
            }
        }
    }

    // For any *'s with 2 numbers, multiply them and sum them
    star_map.values().map(|v| {
        if v.len() == 2 {
            v[0].clone() * v[1].clone()
        } else {
            0
        }
    }).sum()
}
