use std::fs;

fn main() {
    let mut final_score = 0;
    fs::read_to_string("input")
    .unwrap()
    .lines()
    .for_each(|line| {
        let mut number:Vec<String> = Vec::new();
        let line = line
        .replace("nine", "n9n")
        .replace("eight", "e8t")
        .replace("seven", "s7n")
        .replace("six", "s6x")
        .replace("five", "f5e")
        .replace("four", "f4r")
        .replace("three", "t3e")
        .replace("two", "t2o")
        .replace("one", "o1e");
        
         
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
