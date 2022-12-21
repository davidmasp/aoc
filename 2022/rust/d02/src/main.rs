// day 2

use std::io;
// why is this needed?
use std::io::prelude::*;
use std::collections::HashMap;

// rock is 1
// paper is 2
// scissors is 3

fn game_raw(oponent: u32, yours: u32) -> u32 {
    // a bit hardcoded, can do better? 
    if oponent == yours {
        // this means tie
        return 3
    } else if oponent == 3 && yours == 1 {
        // you win
        return 6
    } else if oponent == 1 && yours == 3 {
        // oponent wins
        return 0
    } else if oponent > yours{
        // oponent wins
        return 0
    } else {
        return 6
    }
}

fn get_shape_score(x: &str) -> u32 {
    let shape_score: u32 = match x {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => todo!(),
    };
    return shape_score
}


fn game(oponent: &str,
    yours: &str,
    otoken: &HashMap<&str, u32>,
    ytoken: &HashMap<&str, u32>) -> u32 {
    let sscore = get_shape_score(yours);
    let oponent_int: u32 = match otoken.get(oponent) {
        Some(int_val) => *int_val,
        None => panic!("Wrong Guess value")
    };
    let yours_int: u32 = match ytoken.get(yours) {
        Some(int_val) => *int_val,
        None => panic!("Wrong Guess value")
    };
    let game_score: u32 = game_raw(oponent_int,yours_int);
    return game_score + sscore
}

fn desired_output<'a>(oponent: &'a str, result: &'a str) -> &'a str {
    if result == "Y" {
        let tmp_res: &str = match oponent {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            &_ => todo!(),
        };
        return tmp_res
    } else if result == "Z" {
        // this is win
        let tmp_res: &str = match oponent {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            &_ => todo!(),
        };
        return tmp_res
    } else if result == "X" {
        // this is lost
        let tmp_res: &str = match oponent {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            &_ => todo!(),
        };
        return tmp_res
    } else {
        panic!("AAA");
    }
}

fn game2(oponent: &str,
    result: &str,
    otoken: &HashMap<&str, u32>,
    ytoken: &HashMap<&str, u32>) -> u32 {
    
    let oponent_int: u32 = match otoken.get(oponent) {
        Some(int_val) => *int_val,
        None => panic!("Wrong Guess value")
    };

    let yours_val: &str = desired_output(oponent, result);

    let yours_int: u32 = match ytoken.get(yours_val) {
        Some(int_val) => *int_val,
        None => panic!("Wrong Guess value")
    };

    let game_score: u32 = game_raw(oponent_int,yours_int);

    let sscore = get_shape_score(yours_val);
    return game_score + sscore
}

fn main() {

    println!("Hello, world!");

    let oponent_token = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);

    let you_token = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut total_score: u32 = 0;
    let mut total_score2: u32 = 0;

    // gets the stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // `unwrap` returns a `panic` when it receives a `None`.
        // println!("{}", line.unwrap());
        let this_line = line.unwrap();
        let line_split: Vec<&str> = this_line.split(" ").collect();
        let result_game = game(line_split[0],
            line_split[1],
            &oponent_token,
            &you_token
        );
        total_score += result_game;
        let result_game2 = game2(line_split[0],
            line_split[1],
            &oponent_token,
            &you_token
        );
        total_score2 += result_game2;
    }

    //let test_result: u32 = game("C", "Z");
    println!("RESULT 1: {}", total_score);
    println!("RESULT 2: {}", total_score2);

}
