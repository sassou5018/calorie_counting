use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    //println!("{:#?}", data);
    let mut sum_vec: Vec<i32> = Vec::new();
    let elves: Vec<&str> = data.split("\r\n\r").collect();
    println!("{:#?}", elves);
    for elve in elves.into_iter(){
        let mut sum: i32 = 0;
        // println!("{} ,\n", elve);
        for line in elve.lines(){
            sum = sum + line.parse::<i32>().unwrap_or_else(|_| 0);
        }
        sum_vec.push(sum);
        sum = 0;

    }
    println!("Done, Vec is: {:#?}", sum_vec);

    println!("Max is {}", sum_vec.iter().max().unwrap())
}
