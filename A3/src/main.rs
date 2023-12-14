use std::fs;

fn main() {
    fs::read_to_string("input").unwrap()
    .lines()
    .for_each(|line| {
        let matches: Vec<String> = line.split(";").map(String::from).collect();

        println!("{}", game_id);
    })
}
