//Game predict of number
use std::io::{stdin,Result};
use rand::Rng;
fn main() -> Result<()> {
    let number = vec![0,1,2,3,4,5,6,7,8,9];
    let mut temp_number:Vec<i32> = vec![];
    let mut rng = rand::thread_rng();
    let mut n = 0;
    let mut m = 0;
    let times = 20;
    while n < 4 {
        let rn_number = rng.gen_range(0..10);
        let number_gen = number[rn_number];
        if !temp_number.clone().into_iter().any(|a| a == number_gen){
            temp_number.push(number_gen);
            n += 1;
        }   
    }
    while m < times {
        println!("Remain {} Times", times - m);
        let mut str_in = String::new();
        stdin().read_line(&mut str_in)?;
        let data_in:Vec<i32> = str_in.trim().split(' ').map(|f| f.parse::<i32>()
                            .expect("Incorrect Format , Number Only"))
                            .collect::<Vec<_>>();
        let mut check_format_data = true;
        let clone_data_in = data_in.clone();
        for d in clone_data_in.into_iter(){
            if data_in.clone().into_iter().filter(|p| *p == d).count() > 1 {
                check_format_data = false;
                break;
            }
        }
        if check_format_data{
            let mut count_match_number = 0;
            for d in data_in.clone().into_iter(){
                if temp_number.clone().into_iter().any(|a| a == d){
                    count_match_number += 1;
                }
            }
            let count_check_position_match:i32 = temp_number.clone()
                .into_iter()
                .zip(data_in.into_iter())
                .map(|(a,b)|{
                    let mut count = 0;
                    if a == b {
                        count = 1;
                    }
                    count
            }).sum();
            println!("Correct Number : {} , Correct Position : {} ",count_match_number,count_check_position_match);
            if count_check_position_match == 4 {
                println!("You Won!!!");
                println!("Answer : {:?}" , temp_number);
                break;
            }
            m += 1;
            if m == times {
                println!("Game Over!!!");
                println!("Answer : {:?}" , temp_number);
                break;
            }
        }else{
            println!("Incorrect format");
        }
    }
    Ok(())
}
