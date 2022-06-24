//Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
pub fn bai1() {
    let x = change_value(10, &mut 20);

    println!("{}", x)
}

fn change_value(input: u32, output: &mut u32) -> u32 {
    if input == 1 {
        *output = 3;
    } else {
        *output = 4;
    }

    *output
}

//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố
pub fn bai2() {
    let mut count: u32 = 1;
    let mut num: u64 = 2;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    while count < 10 {
        num += 1;
        if num_is_prime(num) {
            count += 1;
            primes.push(num);
        }
    }
    println!("{:?}", primes);
}

fn num_is_prime(num: u64) -> bool {
    for i in 2..((num as f64).sqrt() as u64) + 1 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// //Exercise 3
// // Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
pub fn bai3() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;

    for n in v.iter_mut() {
        max = std::cmp::max(max, *n);
    }

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}

// //Exercise 4
// // Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// // Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
pub fn bai4() {
    let a = vec![1, 2, 3, 4, 5];
    let a = revert_array(a.to_vec());
    println!("{:?}", a);
}

pub fn revert_array(mut a: Vec<u8>) -> Vec<u8> {
    let mut b: Vec<u8> = Vec::new();
    loop {
        if a.len() == 0 {
            break;
        }
        let d = a.pop().unwrap();
        b.push(d);
    }
    b
}
