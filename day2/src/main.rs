use std::{fs, collections::HashMap};

fn main() {
    let content = fs::read_to_string("src/inp2.txt").unwrap();
    let mut mp: HashMap<String, isize> = HashMap::new();
    let mut sum: usize = 0;
    mp.insert(String::from("blue"), 14);
    mp.insert(String::from("red"), 12);
    mp.insert(String::from("green"), 13);
    let games_info = content.trim().split("\n");
    let mut total = 0;
    for game_info in games_info.into_iter(){
        // println!("{:?} ", game_info);
        let mut sp = game_info.trim().split(":").into_iter();
        let game_id: usize = sp.next().unwrap().replace("Game ", "").trim().parse().unwrap();
        println!("Game id {:?}", game_id);
        total += game_id;
        let game_inf = sp.next().unwrap().trim();
        // println!("Info: {:?}", game_inf);

        let sets = game_inf.split(";").into_iter();
        println!("Sets: ");
        for set in sets{
            // println!(" -> {:?}", set);
            let balls = set.trim().split(", ").into_iter();
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for ball in balls.into_iter(){
                let mut vc = ball.split(" ");
                let cnt: isize = vc.next().unwrap().parse().unwrap();
                let acd = vc.next().unwrap();
                println!("{:?} balls are {:?}", cnt, acd);
                if acd == "red"{
                    red += cnt;
                }else if acd == "blue"{
                    blue += cnt;
                }else if acd == "green"{
                    green += cnt;
                }
            }
            if red > mp["red"]{
                sum += game_id;
                break;
            }else if blue > mp["blue"]{
                sum += game_id;
                break;
            }else if green > mp["green"]{
                sum += game_id;
                break;
            }
        }
    }
    println!("{:?}", total - sum);
}
