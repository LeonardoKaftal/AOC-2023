use std::{fs, collections::HashMap};

fn main() {
    let mut color_map = HashMap::new();
    color_map.insert(String::from("red"), 12);
    color_map.insert(String::from("green"), 13);
    color_map.insert(String::from("blue"), 14);
    let mut total_score = 0;
    let mut game_id = 0;

    fs::read_to_string("input")
    .unwrap()
    .lines()
    .for_each(|line| {
        
        game_id += 1;
        let mut line = String::from(line);
        let mut is_possible = true;
        line.push(';');

        line.drain(..line.find(":").unwrap() + 1);
        let line = line.replace(" ", "");
        let matches: Vec<String> = line.split(";").map(String::from).collect();

        for _match in matches.iter() {
            if !_match.is_empty() {
                let colors = _match.split(",").map(String::from).collect::<Vec<String>>();
                
                for color in colors {
                    let mut color_name = String::new();
                    let mut amount = String::new();
                    for char in color.chars() {
                        if char.is_digit(10) {
                            amount.push(char);
                        }
                        else {
                            color_name.push(char);
                        }
                    }
                    let amount = amount.parse::<i32>().unwrap();
                    let max_value = color_map.get(&color_name).unwrap();
                    if amount > *max_value {
                        is_possible = false;
                        break;
                    }
                }
                
            }
                
        }
        if is_possible {
            
            total_score += game_id;
            println!("Adding for {} so now {}", game_id, total_score);
        }
        is_possible = true;
    })
}
