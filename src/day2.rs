use regex::Regex;

fn day_2_q1() {
    let mut sum: usize = 0;

    let re = Regex::new(r"([A-Za-z0-9]+( [A-Za-z0-9]+)+)").unwrap();
    for line in read_lines("./input/day2.txt"){
        println!("{}",line);        
        let index = line.find(":").unwrap();
        let game_number: usize= line[5..index].parse().unwrap();

        let split = line[index..].split(";");
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for game in split{
            for (_,[number,color]) in re.captures_iter(game).map(|c| c.extract()) {
                let second_index = number.find(" ").unwrap();
                let amount:usize = number[0..second_index].parse().unwrap();
                println!("{}",color); 
                if color == " red" && amount > min_red {
                    min_red = amount;
                }
                if color == " green" && amount > min_green {
                    min_green = amount;
                }
                if color == " blue" && amount > min_blue {
                    min_blue = amount;
                }
            }
        }
        sum += min_red * min_green * min_blue ;
    }

    println!("DONE");
    println!("{}",sum);
}

