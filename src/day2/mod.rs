use std::cmp::max;

fn get_max_per_colour(line: &str) -> (u64, u16, u16, u16) {
    // Remove the punctuation, we don't want it, and split by spaces. Remove the Game prefix
    let clean_line = line.replace(&[':', ',', ';'][..], "");
    let mut parts = clean_line.split_whitespace().skip(1);

    // Parse game ID
    let game_id: u64 = parts
        .next().expect("expected game ID to exist") // extract second element
        .parse().expect("expected game ID to be uint"); // parse the ID

    // Collect to a slice, in chunks of 2 (a number followed by a colour)
    let chunked = &parts.collect::<Vec<&str>>()[..];
    let bag_dips = chunked.chunks_exact(2);

    // Get the max of each colour
    let mut max_red: u16 = 0;
    let mut max_green: u16 = 0;
    let mut max_blue: u16 = 0;
    bag_dips.for_each(|dip| {
        let num_cubes: u16 = dip[0].parse().expect("expected to parse num cubes");
        let colour = dip[1];

        match colour {
            "red" => max_red = max(max_red, num_cubes),
            "green" => max_green = max(max_green, num_cubes),
            "blue" => max_blue = max(max_blue, num_cubes),
            _ => panic!("Shouldn't have another colour")
        };
    });

    (game_id, max_red, max_green, max_blue)
}

pub fn a(inp: &str) -> u64 {
    inp.split('\n').map(|line| {

        let (game_id, max_red, max_green, max_blue) = get_max_per_colour(line);

        // Determine if there are ever more than a colour pulled than should be possible
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            game_id
        } else {
            0
        }
    }).sum()
}

pub fn b(inp: &str) -> u64 {
    inp.split('\n').map(|line| {

        let (_, max_red, max_green, max_blue) = get_max_per_colour(line);

        // Return the power of the minimum set of cubes
        u64::from(max_red * max_green * max_blue)
    }).sum()
}
