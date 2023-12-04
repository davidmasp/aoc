// day 2

// only 12 red cubes, 13 green cubes, and 14 blue cubes

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut total_sum: u32 = 0;
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let mut fewest_hash: HashMap<String, u32> = HashMap::new();
        fewest_hash.insert(
            "red".to_string(),
            0,
        );
        fewest_hash.insert(
            "green".to_string(),
            0,
        );
        fewest_hash.insert(
            "blue".to_string(),
            0,
        );
        let this_line = line.unwrap();
        let first_split: Vec<&str> = this_line.split(':').collect();
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

                if fewest_hash.get(&color_of_cubes).unwrap() < &number_of_cubes {
                    fewest_hash.insert(color_of_cubes, number_of_cubes);
                } else {
                    continue;
                }
            }   
        }

        // calculate power, this is chatgpt magic
        // fold is some sort of map that iterates through a iterator
        // but keeps a consistent variable.
        // intresting.
        let product: u32 = fewest_hash.values().fold(1, |acc, &val| acc * val);
        total_sum += product;
    }

    println!("total sum is: {}",total_sum);
}
