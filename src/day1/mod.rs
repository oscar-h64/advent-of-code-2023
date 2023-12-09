use std::collections::HashMap;

pub fn a(inp: &str) -> u64 {
    inp.split_whitespace().map(|line| {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let first = digits.next().expect("Expected at least 1 digit in line");
        let last = digits.last().unwrap_or(first.clone());

        u64::from((first * 10) + last)
    }).sum()
}

pub fn b(inp: &str) -> u64 {

    let value_map: HashMap<&str, u64> = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9)
    ]);

    inp.split_whitespace().map(|line| {

        let mut first_idx: Option<usize> = None;
        let mut first_val: Option<u64> = None;
        let mut last_idx: Option<usize> = None;
        let mut last_val: Option<u64> = None;

        for (s, v) in &value_map {

            // Look for first match, and if there is no current first index, or this comes before it
            // then update the first index
            let any_match = line.find(s).map(|idx| {
                if first_idx.map_or(true, |curr| curr > idx) {
                    first_idx = Some(idx.clone());
                    first_val = Some(v.clone());
                }
            }).is_some();

            // If there was any match, then do the same from the other end. On this
            if any_match {
                line.rfind(s).map(|idx| {
                    if last_idx.map_or(true, |curr| curr < idx) {
                        last_idx = Some(idx.clone());
                        last_val = Some(v.clone());
                    }
                });
            }
        }

        let fst = first_val.unwrap();
        (fst * 10) + last_val.unwrap_or(fst.clone())
    }).sum()
}
