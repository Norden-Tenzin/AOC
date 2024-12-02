use std::fs;

pub fn problem1(path: &str) -> i32 {
    fn is_increasing_with_max_increment(seq: &[i32], max_step: i32) -> bool {
        seq.windows(2)
            .all(|window| window[0] < window[1] && ((window[0] - window[1]).abs() <= max_step))
    }

    fn is_decreasing_with_max_increment(seq: &[i32], max_step: i32) -> bool {
        seq.windows(2)
            .all(|window| window[0] > window[1] && ((window[0] - window[1]).abs() <= max_step))
    }

    let file = fs::read_to_string(path).expect("unable to read file.");

    let mut res: i32 = 0;
    for line in file.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        if is_increasing_with_max_increment(&levels, 3) {
            res += 1
        } else if is_decreasing_with_max_increment(&levels, 3) {
            res += 1
        }
    }
    return res;
}

pub fn problem2(path: &str) -> i32 {
    fn is_increasing_with_max_increment(seq: &[i32], max_step: i32) -> bool {
        seq.windows(2)
            .all(|window| window[0] < window[1] && ((window[0] - window[1]).abs() <= max_step))
    }

    fn is_decreasing_with_max_increment(seq: &[i32], max_step: i32) -> bool {
        seq.windows(2)
            .all(|window| window[0] > window[1] && ((window[0] - window[1]).abs() <= max_step))
    }

    let file = fs::read_to_string(path).expect("unable to read file.");

    let mut res: i32 = 0;
    for line in file.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|item| item.parse::<i32>().unwrap())
            .collect();
        if is_increasing_with_max_increment(&levels, 3) {
            res += 1
        } else if is_decreasing_with_max_increment(&levels, 3) {
            res += 1
        } else {
            for i in 0..levels.len() {
                let new_levels = vector_without_item(&levels, i);
                if is_increasing_with_max_increment(&new_levels, 3) {
                    res += 1;
                    break;
                } else if is_decreasing_with_max_increment(&new_levels, 3) {
                    res += 1;
                    break;
                }
            }
        }
    }
    return res;
}

fn vector_without_item<T: Clone>(vec: &[T], index: usize) -> Vec<T> {
    vec.iter()
        .enumerate()
        .filter(|&(i, _)| i != index)
        .map(|(_, item)| item.clone())
        .collect()
}
