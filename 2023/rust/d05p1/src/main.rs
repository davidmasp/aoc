
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut seed_values: Vec<u32> = Vec::new();
    let mut order_array: HashMap<String, String> = HashMap::new();
    let mut maps_array: HashMap<(String, String), HashMap<u32, u32>> = HashMap::new();

    let mut first_line = true;
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        if first_line {
            let fields: Vec<&str> = this_line.split(": ").collect();
            let seed_arr: Vec<&str> = fields[1].split(" ").collect();
            for seed_str in seed_arr{
                let seed_i_result: Result<u32, _> = seed_str.parse();
                let seed_i = match seed_i_result{
                    Ok(seed_i) =>  {seed_i},
                    Err(_) => {panic!("Error parsing seed")}
                };

                seed_values.push(seed_i);
            }
            first_line = false;
        }
        // seed-to-soil map:

        let mut current_key = ("".to_string(), "".to_string());
        if this_line.ends_with("map:") {
            let fields: Vec<&str> = this_line.split(" ").collect();
            let seed_arr: Vec<&str> = fields[0].split("-to-").collect();
            let source_key = seed_arr[0].to_string();
            let source_key_clone = source_key.clone();
            let dest_key = seed_arr[1].to_string();
            let dest_key_clone = dest_key.clone();
            order_array.insert(source_key, dest_key);
            current_key = (source_key_clone, dest_key_clone);
            maps_array.insert(current_key, HashMap::new());
        } else {
            let fields: Vec<&str> = this_line.split(" ").collect();
            let dest_start = fields[0];
            let source_start = fields[1];
            let range = fields[2];
            let dest_start_i: u32 =  dest_start.parse().unwrap();
            let source_start_i: u32 =  source_start.parse().unwrap();
            let range_i: u32 = range.parse().unwrap();

            let current_map = maps_array.entry(current_key).or_insert(HashMap::new());

            for i in 0..range_i{
                let dest_key = dest_start_i + i;
                let source_key = source_start_i + i;
                current_map.insert(source_key, dest_key);
            }
        }
    }
    let mut map_counter:usize = 0;
    let mut current_feat = "seed".to_string();
    while map_counter < order_array.len(){
        let target_key =  *order_array.get(&current_feat).unwrap();
        let key_almanac = (current_feat, target_key);
        let trans_hash = maps_array.get(&key_almanac).unwrap();
        
        for (index, value) in seed_values.iter().enumerate() {
            println!("Index: {}, Value: {}", index, value);
            let next_seed_value_option = trans_hash.get(value);
            let next_seed_value = match next_seed_value_option {
                Some(val) => {val.to_owned()},
                None => {value.to_owned()},
            };
            seed_values[index] = next_seed_value;
        }
        current_feat = target_key;
        map_counter += 1;
    }

    let min_value = seed_values.iter().min();

    match min_value {
        Some(min) => println!("The minimum value is {}", min),
        None => println!("The vector is empty"),
    }
}
