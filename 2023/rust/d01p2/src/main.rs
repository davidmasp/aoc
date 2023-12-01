
//day 1

/*
^aaa
see this
https://www.reddit.com/r/adventofcode/comments/1886v2p/comment/kbju0mi/
essentially, my problem is that ...eightwo... should result in 8-2
I used a $ in the regex to only search for the last word in the
buffer text.
*/


use std::io;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    // see issue #aaa
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)$").unwrap();
    let mut current_sum: u32 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let this_line = line.unwrap();

        let mut buffer_txt = "".to_string();

        for c in this_line.chars() {

            buffer_txt.push(c);

            let re_result = re.captures(&buffer_txt);

            let re_num_match = match re_result {
                Some(r) => {
                    let match_str = r.get(0).unwrap().as_str();
                    let match_int:u32 = match match_str {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => 0,
                    };
                    match_int
                },
                None => {
                    0
                },
            };

            let digit_result: Option<u32> = c.to_digit(10);

            let digit_value = match digit_result {
                Some(d) => d,
                None => {
                    0
                },
            };

            if re_num_match != 0 {
                // this means we have found a digit
                // buffer_txt = "".to_string();
                if first_digit == 0 {
                    first_digit = re_num_match;
                } 
                last_digit = re_num_match;
                
            } else if digit_value != 0 {
                if first_digit == 0 {
                    first_digit = digit_value;
                } 
                last_digit = digit_value;  
            } else if digit_value != 0 && re_num_match != 0 {
                // this means we are f***ed
                println!("{}", buffer_txt);
                panic!("we are f***ed");
            } else {
                continue;
            }
        };

        let two_dig = format!("{}{}", first_digit, last_digit);
        //println!("{}, {}", &two_dig, &this_line );
        //println!("two dig: {}", two_dig);
        current_sum += two_dig.parse::<u32>().unwrap();
    }
    
    println!("sum: {}", current_sum);

}


