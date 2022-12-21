
use std::io;
use std::io::prelude::*;
use std::convert::TryFrom;

// use std::collections::HashMap;

// why not? https://stackoverflow.com/a/70194530
use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Crate {
    name: String,
    position: usize, // is this worth?
    pile: i32,
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.pile == other.pile {
            return Some(other.position.cmp(&self.position));
        }
        Some(self.pile.cmp(&other.pile))
    }
}

// this is to get the output
fn get_top_crates(bay: Vec<Crate>) -> String {
    //bay.sort();
    let mut top_sels: Vec<String> = Vec::new();
    let mut curr_pile: i32 = 0;
    for x in bay {
        if x.pile == curr_pile {
            continue
        } else {
            top_sels.push(x.name);
            curr_pile = x.pile;
        }
    }

    return top_sels.join("");
}


fn build_bay(lines: Vec<String>) -> Vec<Crate> {
    let max_pos: usize = lines.len() - 1;

    // this is the last?
    let pile_line: &String = &lines[lines.len() - 1];
    let line_split: Vec<&str> = pile_line.trim().split("   ").collect();
    let pile_num: Vec<i32> = line_split.into_iter()
              .map(|x| {
                let my_string = x.to_string();  
                let my_int = my_string.parse::<i32>().unwrap();
                my_int
                })
               .collect();

    let mut line_idx: Vec<usize> = Vec::new();
    let mut init: usize = 1;
    for _i in &pile_num{
        line_idx.push(init);
        init += 4
    }

    //println!("line_idx {:?}", line_idx);
    
    let mut list_of_crates: Vec<Crate> = Vec::new();

    for i in 0..max_pos {
        let str_inst: &String = &lines[i];
        let mut line_split: Vec<&str> = str_inst.split("").collect();
        line_split.remove(0);
        line_split.remove(line_split.len()-1);
        //println!("line split {:?}", line_split);
        for idx in 0..line_split.len(){
            if line_split[idx] == " " || !line_idx.contains(&idx) {
                continue;
            } else {
                let index = line_idx.iter().position(|&r| r == idx).unwrap();
                let tmp_pile = &pile_num[index];
                let tmp_crate: Crate = Crate{
                    name: line_split[idx].to_string(),
                    position: max_pos - i, ////mmmmmmmh
                    pile: *tmp_pile,
                };
                list_of_crates.push(tmp_crate);
            }
        }
    }

    return list_of_crates;
}


fn main() {

    let mut is_header: bool = true;
    let stdin = io::stdin();

    let mut header: Vec<String> = Vec::new();

    //three crates are moved from stack 1 to stack 3
    let mut com_unit: Vec<usize> = Vec::new();
    let mut com_from: Vec<i32> = Vec::new();
    let mut com_to: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        if this_line.is_empty() {
            // this breaks the current interaction and breaks the header
            is_header = false;
            continue
        }

        if is_header {
            let line_inst: String = this_line.to_string().clone();
            header.push(line_inst);
        } else {
            let line_inst: String = this_line.to_string().clone();
            let line_split: Vec<&str> = line_inst.split(" ").collect();
            //
            let str_unit = line_split[1].to_string();
            let n_unit = str_unit.parse::<i32>().unwrap();
            let inst_unit: usize = usize::try_from(n_unit).unwrap();
            com_unit.push(inst_unit);
            //
            let str_from = line_split[3].to_string();
            let inst_from = str_from.parse::<i32>().unwrap();
            //let inst_from: usize = usize::try_from(n_from).unwrap();
            com_from.push(inst_from);
            //
            let str_to = line_split[5].to_string();
            let inst_to = str_to.parse::<i32>().unwrap();
            //let inst_to: usize = usize::try_from(n_to).unwrap();
            com_to.push(inst_to);
        }
        
    }

    //println!("{:?}", com_to);
    let mut bay: Vec<Crate> = build_bay(header);      
    bay.sort();

    // here i need to clone stuff
    let mut bay2:  Vec<Crate> = bay.clone();

    for idx in 0..com_to.len(){
        let n_inst = com_unit[idx];
        let source = com_from[idx];
        let to = com_to[idx];

        for _step in 0..n_inst {
            let mut max_pos: usize = 0;
            // detect max pos of pile to
            for y in &bay {
                if y.pile == to {
                    max_pos = y.position;
                    break
                }
            }
            // select the crate
            for x in &mut bay {
                if x.pile == source {
                    x.position = max_pos + 1;
                    x.pile = to;
                    break
                }
            }
            bay.sort();
        }
    }
    bay.sort();
    let final_results: String = get_top_crates(bay);
    
    println!("FINAL (1) -> {}",final_results);

    // here is need to take n crates at the same time
    for idx in 0..com_to.len(){
        let n_inst = com_unit[idx];
        let mut moves_left: i32 = i32::try_from(n_inst).unwrap();
        let source = com_from[idx];
        let to = com_to[idx];

        let mut max_pos: usize = 0;
        // detect max pos of pile to
        for y in &bay2 {
            if y.pile == to {
                max_pos = y.position;
                break
            }
        }

        // select the crate
        for x in &mut bay2 {
            if x.pile == source {
                x.position = max_pos + n_inst;
                moves_left = moves_left -1;
                x.pile = to;
                if moves_left == 0{
                    break
                }
            }
        }
        bay2.sort();
    }
    bay2.sort();
    let final_results: String = get_top_crates(bay2);
    println!("FINAL (2) -> {}",final_results);
}
