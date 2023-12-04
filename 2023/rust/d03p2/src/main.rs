

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn add_number_to_map (matrix: &mut HashMap<(usize, usize, usize, usize), u32>, digits: &Vec<u8>, start_pos: (usize, usize), end_pos: (usize, usize) ) -> () {
    // this is copilot
    let mut number = 0;
    for &digit in digits.iter() {
        number = number * 10 + digit as u32;
    }
    let start_pos_x = start_pos.0;
    let start_pos_y = start_pos.1;
    let end_pos_x = end_pos.0;
    let end_pos_y = end_pos.1;
    matrix.insert((start_pos_x, start_pos_y, end_pos_x, end_pos_y),
                    number);
}

fn main() {
    let stdin = io::stdin();

    let mut matrix_symbols: HashMap<(usize,usize), String> =  HashMap::new();
    // here there is a start, and finish of the number.
    let mut matrix_numbers: HashMap<(usize, usize, usize, usize), u32> =  HashMap::new();
    let mut idy: usize = 0;
    let mut active_number = false;
    // i need to start this values because the compiler cannot tell
    // that the active_number will always be false in the first iteration
    
    for line in stdin.lock().lines() {
        let this_line: String = line.unwrap();
        let mut start_number: (usize, usize) = (0, idy);
        let mut number_digits: Vec<u8> = Vec::new();
        let mut last_idx: usize = 0;
        for idx in 0..this_line.len() {
            let character = &this_line[idx..idx+1];
            if character == "." {
                if active_number {
                    // here number is done
                    let end_pos = (idx-1, idy);
                    add_number_to_map(&mut matrix_numbers, &number_digits, start_number, end_pos);
                    active_number = false;
                    number_digits.clear();
                    continue;
                } else {
                    continue;
                }
            }
            let digit_result: Result<u8, _> = character.parse();
            match digit_result {
                Ok(digit) => {
                    if active_number {
                        number_digits.push(digit);
                    } else {
                        start_number.0 = idx;
                        number_digits.push(digit);
                        active_number = true;
                        //println!("{:?}", start_number);
                    }
                },
                Err(_) => {
                    // here number is done
                    if active_number {
                        let end_pos = (idx-1, idy);
                        add_number_to_map(&mut matrix_numbers, &number_digits, start_number, end_pos);
                        active_number = false;
                        number_digits.clear();
                    }

                    let key = (idx, idy);
                    matrix_symbols.insert(
                        key,
                        character.to_string(),
                    );
                } 
            }
            last_idx = idx;
        }

        if active_number {
            // this is when the line has finished
            let end_pos = (last_idx, idy);
            if end_pos.1 != start_number.1 {
                panic!("aaaas")
            }
            add_number_to_map(&mut matrix_numbers, &number_digits, start_number, end_pos);
            active_number = false;
            number_digits.clear();
        }

        idy += 1;
    } 

    let mut gear_hash : HashMap<(usize,usize), Vec<u32>> =  HashMap::new();
    for i in matrix_numbers.keys(){
        let sel_number_ref = matrix_numbers.get(i).unwrap();
        let start_x = i.0;
        let start_y = i.1;
        let end_x = i.2;
        let end_y = i.3;
        if end_y != start_y {
            println!("start x {} y {}, end x {} y {}", start_x, start_y, end_x, end_y);
            panic!("Wrong end Y?")
        }
        for k in start_x..(end_x+1) {
            //println!("{}", k);
            // each point has 8 possible interactions
            if start_y > 0 {
                let key_top = (k, start_y-1);

                if matrix_symbols.contains_key(&key_top)  {
                    // i am not sure i need to de-reference here.
                    let symbol = matrix_symbols.get(&key_top).unwrap();
                    if symbol.as_str() == "*" {
                        let gh_vec = gear_hash.entry(key_top).or_insert(Vec::new());
                        gh_vec.push(*sel_number_ref);
                    }
                    break;
                } 
            
                let key_topright = (k+1, start_y-1);

                if matrix_symbols.contains_key(&key_topright)  {
                    // i am not sure i need to de-reference here.
                    let symbol = matrix_symbols.get(&key_topright).unwrap();
                    if symbol.as_str() == "*" {
                        let gh_vec = gear_hash.entry(key_topright).or_insert(Vec::new());
                        gh_vec.push(*sel_number_ref);
                    }
                    break;
                } 
            
            }

            if start_x > 0 && start_y > 0 {
                let key_topleft = (k-1, start_y-1);
                if matrix_symbols.contains_key(&key_topleft)  {
                    // i am not sure i need to de-reference here.
                    let symbol = matrix_symbols.get(&key_topleft).unwrap();
                    if symbol.as_str() == "*" {
                        let gh_vec = gear_hash.entry(key_topleft).or_insert(Vec::new());
                        gh_vec.push(*sel_number_ref);
                    }
                    break;
                } 
            } 

            if start_x > 0 {
                let key_left = (k-1, start_y);
                if matrix_symbols.contains_key(&key_left) {
                    // i am not sure i need to de-reference here.
                    let symbol = matrix_symbols.get(&key_left).unwrap();
                    if symbol.as_str() == "*" {
                        let gh_vec = gear_hash.entry(key_left).or_insert(Vec::new());
                        gh_vec.push(*sel_number_ref);
                    }
                    break;
                }

                let key_bottomleft = (k-1, start_y+1);
                if matrix_symbols.contains_key(&key_bottomleft) {
                    // i am not sure i need to de-reference here.
                    let symbol = matrix_symbols.get(&key_bottomleft).unwrap();
                    if symbol.as_str() == "*" {
                        let gh_vec = gear_hash.entry(key_bottomleft).or_insert(Vec::new());
                        gh_vec.push(*sel_number_ref);
                    }
                    break;
                }
            }
            
            
            let key_right = (k+1, start_y);
            
            if matrix_symbols.contains_key(&key_right)  {
                // i am not sure i need to de-reference here.
                let symbol = matrix_symbols.get(&key_right).unwrap();
                if symbol.as_str() == "*" {
                    let gh_vec = gear_hash.entry(key_right).or_insert(Vec::new());
                    gh_vec.push(*sel_number_ref);
                }
                break;
            }

            let key_bottom = (k, start_y+1);
            
            if matrix_symbols.contains_key(&key_bottom)  {
                // i am not sure i need to de-reference here.
                let symbol = matrix_symbols.get(&key_bottom).unwrap();
                if symbol.as_str() == "*" {
                    let gh_vec = gear_hash.entry(key_bottom).or_insert(Vec::new());
                    gh_vec.push(*sel_number_ref);
                }
                break;
            }
            
            let key_bottomright = (k+1, start_y+1);
            
            if matrix_symbols.contains_key(&key_bottomright)  {
                // i am not sure i need to de-reference here.
                let symbol = matrix_symbols.get(&key_bottomright).unwrap();
                if symbol.as_str() == "*" {
                    let gh_vec = gear_hash.entry(key_bottomright).or_insert(Vec::new());
                    gh_vec.push(*sel_number_ref);
                }
                break;
            }

        }
    }

    let mut selected_numbers : Vec<u32> = Vec::new();
    for gear in gear_hash.keys(){
        let sel_vec = gear_hash.get(gear).unwrap();
        if sel_vec.len() == 2{
            let num = sel_vec.get(0).unwrap() * sel_vec.get(1).unwrap();
            selected_numbers.push(num);
        }
    }

    // here is sum all the numbers
    println!("{:?}",selected_numbers);
    let sum: u32 = selected_numbers.iter().fold(0, |acc, &num| acc + num);

    println!("Total sum: {}", sum);

}
