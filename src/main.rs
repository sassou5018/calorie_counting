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
    let mut clone_vec = sum_vec.clone();
    let mut top_vec: Vec<i32> = Vec::new();
    for i in 0..3{
        let y = clone_vec.iter().max().unwrap();
        let x = clone_vec.iter().position(|&r| r== y.clone()).unwrap();

        top_vec.push(y.clone());
        clone_vec.remove(x);
    }
    println!("top vec: {:#?}", top_vec);
    println!("Sum of top 3: {}", top_vec.into_iter().sum::<i32>())
}
