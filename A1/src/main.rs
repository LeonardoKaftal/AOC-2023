use std::fs;

fn main() {

    let mut numbers: Vec<i32> = vec![];
    fs::read_to_string("input")
    .unwrap()
    .lines()
    .for_each(|line| {
        let mut number = String::new();
        let mut is_adding = false;

        for digit in line.chars() {
            if is_adding {
                break;
            }
            else if digit.is_numeric() {
                is_adding = true;
                number.push(digit);
            }
            
           
        }

        is_adding = false;

        for digit in line.chars().rev() {
            if is_adding {
                break;
            }
            else if digit.is_numeric() {
                is_adding = true;
                number.push(digit);
            }
        }
        numbers.push(number.parse::<i32>().unwrap())
    });

    let sum: i32 = numbers.iter().sum();
    println!("{}", sum)
}
