use std::fs;

pub fn run_01() -> i32 {
    let mut cumsum = 0;
    'line_loop: for (i, line) in fs::read_to_string("./src/d02/input.txt").unwrap().lines().map(String::from).enumerate() {
        let relevations_start_index = line
                .chars()
                .position(|ch| ch == ':')
                .unwrap();
        let revelations_str: String = line
                .chars()
                .skip(relevations_start_index + 2)
                .collect();

        for part in revelations_str.split("; ") {
            for cube_desc in part.split(", ") {
                let phrase = cube_desc.split(" ").collect::<Vec<&str>>();
                let amt = phrase[0].parse::<i32>().unwrap();
                let color = phrase[1];

                let valid = match color {
                    "red" => amt <= 12,
                    "green" => amt <= 13,
                    "blue" => amt <= 14,
                    _ => panic!(),
                };

                if !valid {
                    continue 'line_loop;
                }
            }
        }

        cumsum += (i as i32) + 1;
    }

    cumsum
}

pub fn run_02() -> i32 {
    let mut cumsum = 0;
    for line in fs::read_to_string("./src/d02/input.txt").unwrap().lines().map(String::from) {
        let relevations_start_index = line
                .chars()
                .position(|ch| ch == ':')
                .unwrap();
        let revelations_str: String = line
                .chars()
                .skip(relevations_start_index + 2)
                .collect();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for part in revelations_str.split("; ") {
            for cube_desc in part.split(", ") {
                let phrase = cube_desc.split(" ").collect::<Vec<&str>>();
                let amt = phrase[0].parse::<i32>().unwrap();
                let color = phrase[1];

                match color {
                    "red" => max_red = amt.max(max_red),
                    "green" => max_green = amt.max(max_green),
                    "blue" => max_blue = amt.max(max_blue),
                    _ => panic!(),
                };
            }
        }

        cumsum += max_red * max_green * max_blue;
    }

    cumsum
}