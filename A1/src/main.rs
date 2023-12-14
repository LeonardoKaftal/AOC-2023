use std::fs;

fn main() {
    let mut final_score = 0;
    fs::read_to_string("input")
    .unwrap()
    .lines()
    .for_each(|line| {
        let mut number:Vec<String> = Vec::new();
        for digit in line.chars() {
            if digit.is_digit(10) {
                number.push(String::from(digit));
            }
        }
        final_score +=  (number[0].clone() + number[number.len() - 1].as_str()).parse::<i32>().unwrap();
        number.clear();
    });

    println!("{}", final_score)
    
}
