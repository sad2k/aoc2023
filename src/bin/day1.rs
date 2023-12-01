use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = contents.lines();

    let mut res = 0;
    for line in lines {
        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect();
        res += digits[0] * 10 + digits[digits.len() - 1];
    }
    println!("{}", res);
}
