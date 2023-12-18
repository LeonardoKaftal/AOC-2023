use std::{fs};


fn main() {
    let mut global_score = 0;
    fs::read_to_string("input")
    .unwrap()
    .lines()
    .for_each(|line| {
        let mut line = line.replace(" ", "").trim().to_string();
        // remove the game ID
        line.drain(..line.find(":").unwrap() + 1);

        let mut red_combinations = Vec::new();
        let mut green_combinations = Vec::new();
        let mut blue_combinations = Vec::new();

        for matches in line.split(";") {
            let mut color_name = String::new();
            let mut amount = String::new();

            for color in matches.split(",") {
                for char in color.chars() {
                    if !char.is_digit(10) {
                        color_name.push(char);
                    }
                    else {
                        amount.push(char);
                    }
                }
                let parsed_amount = amount.parse::<i32>().unwrap();
                match color_name.as_str() {
                    "red" => red_combinations.push(parsed_amount),
                    "green" => green_combinations.push(parsed_amount),
                    "blue" => blue_combinations.push(parsed_amount),
                    _=>  println!("Something went wrong because of {} e amount {}", color_name, amount)
                };
                color_name.clear();
                amount = String::new();
            }
        }
        red_combinations.sort();
        green_combinations.sort();
        blue_combinations.sort();

        let best_red_move: i32 = red_combinations.last().unwrap().abs();
        let best_green_move: i32 = green_combinations.last().unwrap().abs();
        let best_blue_move: i32 = blue_combinations.last().unwrap().abs();

        global_score += best_red_move * best_green_move * best_blue_move;

        red_combinations.clear();
        green_combinations.clear();
        blue_combinations.clear();
    });

    println!("{}", global_score)
}