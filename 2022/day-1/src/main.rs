use std::fs;
use std::env;
use std::cmp;
use std::num::ParseIntError;

fn main() {
    // get the input file from argument
    let args:Vec<String> = env::args().collect();
    let file_path:&String = &args[1];
    println!("Reading File: {}",file_path);
    
    // get the calories
    let contents:Vec<Result<u16,ParseIntError>> = fs::read_to_string(file_path)
                            .unwrap()
                            .lines()
                            .map(|line| line.parse::<u16>())
                            .collect();
    
    let mut max_monkey_value:u32 = 0;
    let mut curr_sum:u32 = 0;
    // increment curr sum untill err occurs meaning new line
    // then compare if curr sum is greater than previously known
    // max monkey value and keep note of  whichever is max
    for number in contents{
        match number{
            Ok(num)=>{
                curr_sum += num as u32;
            }
            Err(_) =>{
                max_monkey_value = cmp::max(curr_sum,max_monkey_value);
                curr_sum = 0;
            }
        }
    }
    println!("Max Calories is {} clories",max_monkey_value)
}
