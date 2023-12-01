use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let file_path: &str = "src/input.txt";
    println!("Reading file {}", file_path);

    let file_contents: String = fs::read_to_string(file_path).unwrap();
    
    let mut sum = 0;
    const RADIX: u32 = 10;
    // let contents = ["7pqrstsixteen", "4nineeightseven2", "xtwone3four"];


    for part in file_contents.split("\n") {
        println!("{}", part);
        let mut num = 0;
        for c in part.chars(){
            if c.is_numeric(){
                num = c.to_digit(RADIX).unwrap() * 10;
                break;
            }
        }
        println!("first digit");
        for c in part.chars().rev(){
            if c.is_numeric(){
                num += c.to_digit(RADIX).unwrap();
                break;
            }
        }
        println!("second digit");
        println!("Num: {}",num);
        sum += num;
    }
    println!("{}", sum);

    let nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut mp = HashMap::new();
    let mut jk = 1;
    for n in nums.clone(){
        mp.insert(n, jk);
        jk += 1;
    }
    for part in file_contents.split("\n"){
    // for part in contents{
        println!("{}", part.trim());
        let mut num = 0;
        let mut i = 0;
        for c in part.chars(){
            println!("{}", part);
            if part.trim().len() == 0 {
                break;
            }
            if c.is_numeric(){
                num = c.to_digit(RADIX).unwrap() * 10;
                println!("Numeric");
                break;
            }else{
                let mut wrd = "";
                let mut a: bool = false;
                for k in i..i+6{
                    println!("{} {}", k, part.len());
                    if k >= part.len(){
                        println!("break....");
                        break;
                    }
                    wrd = &part[i..=k];
                    println!("{}",wrd);
                    if nums.contains(&wrd){
                        num = mp[wrd] * 10;
                        println!("READ {} as {}", wrd, num);
                        a = true;
                        break;
                    }
                }
                if a{
                    break;
                }
            }
            i+=1
        }
        println!("first digit {}", num);
        i = part.len();
        i -= 1;
        for c in part.chars().rev(){
            println!("i -> {}",i);
            if c.is_numeric(){
                num += c.to_digit(RADIX).unwrap();
                println!("Numeric");
                break;
            }else{
                println!("In word");
                let mut wrd = "";
                let mut a:bool = false;
                for k in i..i+6{
                    println!("k {}", k);
                    if k >= part.len(){
                        println!("break....");
                        break;
                    }
                    wrd = &part[i..=k];
                    println!("{}", wrd);
                    if nums.contains(&wrd){
                        num += mp[wrd];
                        println!("SECOND READ {} as {} at {}", wrd, num, k);
                        a = true;
                        break;
                    }
                }
                if a{
                    break;
                }
            }
            i-=1;
        }
        println!("Num: {}",num);
        sum += num;
        println!("Sum : {}", sum);

    }
    println!("{}", sum);
}
