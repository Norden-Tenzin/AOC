use std::collections::HashMap;
use std::fs;

pub fn problem1(path: &str) -> i32 {
    let file = fs::read_to_string(path).expect("Unable to read file.");
    let mut first_lst: Vec<i32> = Vec::new();
    let mut second_lst: Vec<i32> = Vec::new();

    let mut data: Vec<(i32, i32)> = Vec::new();

    for line in file.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            first_lst.push(first.parse::<i32>().unwrap());
            second_lst.push(second.parse::<i32>().unwrap());
        }
    }

    first_lst.sort();
    second_lst.sort();

    for i in 0..first_lst.len() {
        data.push((first_lst[i], second_lst[i]))
    }

    let mut res: i32 = 0;
    for single in data.iter() {
        let (first, second) = single;
        let diff = (first - second).abs();
        res += diff;
    }

    return res;
}

pub fn problem2(path: &str) {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut first_lst: Vec<i32> = Vec::new();
    let mut second_lst: Vec<i32> = Vec::new();

    for line in file.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            first_lst.push(first.parse::<i32>().unwrap());
            second_lst.push(second.parse::<i32>().unwrap());
        }
    }

    let mut second_frequecy: HashMap<i32, i32> = HashMap::new();
    for &num in second_lst.iter() {
        *second_frequecy.entry(num).or_insert(0) += 1;
    }

    let mut res: i32 = 0;
    for num in first_lst.iter() {
        res += num * second_frequecy.get(num).cloned().unwrap_or(0);
    }

    println!("{}", res);
}
