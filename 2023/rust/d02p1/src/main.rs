// day 2

// only 12 red cubes, 13 green cubes, and 14 blue cubes

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut total_sum: u32 = 0;

    let mut cube_balls: HashMap<String, u32> = HashMap::new();
    cube_balls.insert(
        "red".to_string(),
        12,
    );
    cube_balls.insert(
        "green".to_string(),
        13,
    );
    cube_balls.insert(
        "blue".to_string(),
        14,
    );
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let mut is_game_valid = true;

        let this_line = line.unwrap();
        let first_split: Vec<&str> = this_line.split(':').collect();
        let game_id = first_split[0].to_string();
        let game_id_split: Vec<&str> = game_id.split(' ').collect();
        let game_id_num: u32 = game_id_split[1].to_string().parse().unwrap();
        let second_split: Vec<&str> = first_split[1].split(';').collect();
        for i in 0..second_split.len() {
            let tmp_str = second_split[i].to_string();
            let third_split: Vec<&str> = tmp_str.split(",").collect();
            for j in 0..third_split.len(){
                let draw_string = third_split[j].to_string();
                println!("draw: {}", draw_string);
                let forth_split: Vec<&str> = draw_string.split(" ").collect();
                let number_of_cubes: u32 = forth_split[1].parse().unwrap();
                let color_of_cubes = forth_split[2].to_string();
                if cube_balls.get(&color_of_cubes).unwrap() < &number_of_cubes {
                    is_game_valid = false;
                    continue;
                }
            }   
        }
        if is_game_valid {
            total_sum += game_id_num;
        }
    }

    println!("total sum is: {}",total_sum);
}
