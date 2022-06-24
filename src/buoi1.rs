use std::fs;
use std::io;
pub fn bai1() {
    let org_arg = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sub_arr = [6, 8, 10];

    let mut count: i32 = 0;

    for org in org_arg {
        for sub in sub_arr {
            if org == sub {
                count += 1;
            }
        }
    }
    if count == sub_arr.len() as i32 {
        print!("Sub arr is child");
    } else {
        print!("Sub arr is not child")
    }
}

pub fn bai2() {
    let contents = fs::read_to_string("./1-s2.0-S0960982203005347-mmc6.txt")
        .expect("Something went wrong reading the file");

    println!("Input the char you want to count");

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    let mut count = 0;

    for c in contents.chars() {
        count = count
            + if c.to_string().eq(&input_string.trim()) {
                1
            } else {
                0
            }
    }

    print!("Total of {} is {}", input_string.trim(), count)
}
