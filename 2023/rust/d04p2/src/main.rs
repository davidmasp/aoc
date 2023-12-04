
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn calculate_line_points(win_num: Vec<u32>, my_num: Vec<u32>) -> u32 {

    let mut line_points: u32  = 0;
    for i in my_num {
        if win_num.contains(&i) {
            line_points += 1;
        } else {
            continue;
        }
    }
    line_points
}

fn extract_numbers(s: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    let nums: Vec<u32> = re.captures_iter(s)
        .filter_map(|cap| cap[0].parse::<u32>().ok())
        .collect();
    nums
}

fn parse_line(line: String) -> (u32, Vec<u32>, Vec<u32>){
    
    let fields: Vec<&str> =  line.split(":").collect();
    let card_str = fields[0];
    let nums = extract_numbers(card_str);
    if nums.len() > 1{
        panic!("more than one numbers")
    }
    let card_number = nums[0];

    let numbers: Vec<&str> = fields[1].split("|").collect();
    let winning_numbers = numbers[0];
    let my_numbers = numbers[1];
    let my_vec = parse_numbers(my_numbers);
    let win_vec = parse_numbers(winning_numbers);

    (card_number, win_vec, my_vec)

}

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

    let mut points_hash : HashMap<u32, u32> = HashMap::new();
    let mut times_hash: HashMap<u32, u32> =   HashMap::new();

    let mut max_card_id: u32 = 0;

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let values = parse_line(this_line);
        let points = calculate_line_points(values.1, values.2);
        points_hash.insert(values.0, points);
        // this represents the original copy
        times_hash.insert(values.0, 1);
        max_card_id = values.0;
    }

    let mut total_points: u32 = 0;

    for card_id in 1..max_card_id+1 {
        let times_inst = times_hash.get(&card_id).unwrap();
        total_points += times_inst;
        let points_inst = points_hash.get(&card_id).unwrap();
        for _time_inst in 0..*times_inst {
            for idx in (card_id + 1)..(card_id + points_inst + 1 ){
                if let Some(value) = times_hash.get_mut(&idx) {
                    *value += 1;
                }
            }
        }
    }

    println!("total points: {}", total_points);


}
