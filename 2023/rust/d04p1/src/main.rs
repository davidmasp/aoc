

use std::io;
use std::io::prelude::*;

fn parse_numbers(numbers_line: &str) -> Vec<u32> {

    let mut result_vec: Vec<u32> = Vec::new();
    // the trick of this problem is to parse the numbers, sometimes
    // we have things like " 6" and "32".
    let mut total_len = numbers_line.len();

    // 1 space
    // 2 number
    // 1 space
    // ..
    if (total_len % 3) != 0 {
        total_len -= 1;
    }

    if (total_len % 3) != 0 {
        panic!("Error in parsing")
    }

    let n_numbers = total_len / 3;
    for i in 0..n_numbers{
        let idx = (i * 3) + 1;
        let idx2 = idx + 2;
        let num_str = numbers_line[idx..idx2].to_string();
        let trimmed_str = num_str.trim_start().to_string();
        println!("nstring = {}", trimmed_str);
        let num_int_result: Result<u32, _> = trimmed_str.parse();
        match num_int_result {
            Ok(num_int) => {result_vec.push(num_int);},
            Err(_) => {panic!("not a number?")} 
        };
    }

    result_vec
}



fn main() {
    let stdin = io::stdin();
    let mut total_points:u32 = 0;
    for line in stdin.lock().lines() {
        let this_line: String = line.unwrap();
        let mut first_number = true;
        let mut line_points: u32  = 0;
        let fields: Vec<&str> =  this_line.split(":").collect();
        let numbers: Vec<&str> = fields[1].split("|").collect();
        let winning_numbers = numbers[0];
        let my_numbers = numbers[1];
        let my_vec = parse_numbers(my_numbers);
        let win_vec = parse_numbers(winning_numbers);

        for i in my_vec{
            if win_vec.contains(&i) {
                if first_number {
                    line_points += 1;
                    first_number = false;
                } else {
                    line_points = line_points * 2;
                }
            } else {
                continue;
            }
        }

        total_points += line_points;
    }

    println!("total sum: {}", total_points);

}
