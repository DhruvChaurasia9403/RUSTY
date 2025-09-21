use std::io;
use rand::prelude::*;

fn main() {
    println!("Lets start the Guessing game, There are 6 fruits name and you have to guess which one has rust choosen");
    let mut rng = rand::thread_rng();
    let ls = ["Banana","Orange","apple","Mango","Peach","Strwberrys","Kiwi"];
    let index = rng.gen_range(0..ls.len());
    let rusted_fruit = ls[index];
    let mut attempts  = 0;
    loop{
        if attempts>=5{
            println!();
            println!("You are out of luck, the correct answer was : {}",rusted_fruit);
            break;
        }
        let mut input:String = String::new();
        match io::stdin().read_line(&mut input){ 
        Ok(_)=>{
            let selected_fruit = input.trim().to_lowercase();
            // println!("You selected Fruit :{}",selected_fruit);
            
            if !ls.iter().any(|&f| f.to_lowercase() == selected_fruit){
                println!("Fruit entered is not found");
                attempts+=1;
                let mut left = 5-attempts;
                println!("Total attemps left : {}",left);
                println!();
                continue;
            }
            if check(&selected_fruit,rusted_fruit){
                attempts=attempts+1;
                println!("Yay!, you guessed it right");
                println!();
                break;
            }
            else{
                attempts+=1;
                let mut left = 5-attempts;
                println!("Retry");
                println!("Total attemps left : {}",left);
                println!();
            }
        }
        Err(e)=>{
            println!("Error:{}",e);
        }
    }
}


}
fn check(selected_fruit:&str , rusted_fruit:&str)-> bool{
    return selected_fruit.eq_ignore_ascii_case(rusted_fruit);
}
