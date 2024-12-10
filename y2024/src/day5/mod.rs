use std::collections::{HashMap, HashSet};
use std::fs;

pub fn problem1(path: &str) -> usize {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut is_data = false;
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    let mut invalid_data: Vec<Vec<&str>> = Vec::new();
    let mut res: usize = 0;

    for line in file.lines() {
        if is_data {
            data.push(line.split(",").collect());
        } else {
            if line == "" {
                is_data = true;
                continue;
            }
            if let Some((left, right)) = line.split_once("|") {
                rules
                    .entry(left.to_string())
                    .or_insert(HashSet::new())
                    .insert(right.to_string());
            }
        }
    }

    //println!("{:?}", rules);

    for pages in &data {
        if is_right_order(&pages, &rules) {
            let mid = pages.len() / 2;
            res += pages[mid].parse::<usize>().unwrap();
        } else {
            invalid_data.push(pages.clone());
        }
    }

    fn is_right_order(pages: &Vec<&str>, rules: &HashMap<String, HashSet<String>>) -> bool {
        let mut cur = 0;
        let mut ptr = 1;
        while cur < pages.len() {
            while ptr < pages.len() {
                let res = check_if_valid(pages[cur], pages[ptr], &rules);
                if res == false {
                    return false;
                }
                ptr += 1;
            }
            cur += 1;
            ptr = cur + 1;
        }
        return true;
    }

    fn check_if_valid(left: &str, right: &str, rules: &HashMap<String, HashSet<String>>) -> bool {
        if rules.get(left).unwrap_or(&HashSet::new()).contains(right) {
            if rules.get(right).unwrap_or(&HashSet::new()).contains(left) {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
    return res;
}

pub fn problem2(path: &str) -> usize {
    let file = fs::read_to_string(path).expect("unable to read file.");
    let mut is_data = false;
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
    let mut data: Vec<Vec<&str>> = Vec::new();
    let mut invalid_data: Vec<Vec<&str>> = Vec::new();
    let mut res: usize = 0;

    for line in file.lines() {
        if is_data {
            data.push(line.split(",").collect());
        } else {
            if line == "" {
                is_data = true;
                continue;
            }
            if let Some((left, right)) = line.split_once("|") {
                rules
                    .entry(left.to_string())
                    .or_insert(HashSet::new())
                    .insert(right.to_string());
            }
        }
    }

    for pages in &data {
        if !is_right_order(&pages, &rules) {
            invalid_data.push(pages.clone());
        }
    }

    fn is_right_order(pages: &Vec<&str>, rules: &HashMap<String, HashSet<String>>) -> bool {
        let mut cur = 0;
        let mut ptr = 1;
        while cur < pages.len() {
            while ptr < pages.len() {
                let res = check_if_valid(pages[cur], pages[ptr], &rules);
                if res == false {
                    return false;
                }
                ptr += 1;
            }
            cur += 1;
            ptr = cur + 1;
        }
        return true;
    }

    fn check_if_valid(left: &str, right: &str, rules: &HashMap<String, HashSet<String>>) -> bool {
        if rules.get(left).unwrap_or(&HashSet::new()).contains(right) {
            if rules.get(right).unwrap_or(&HashSet::new()).contains(left) {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    let mut all_valid_pages: Vec<Vec<&str>> = Vec::new();
    for pages in invalid_data {
        let mut valid_pages: Vec<&str> = Vec::new();
        let mut buffer: Vec<&str> = pages.clone();
        let mut cur: usize = 0;
        while cur < buffer.len() {
            //println!("p: {:?}", pages);
            //println!("b: {:?}", buffer);
            //println!("{}", cur);

            let curr_page = buffer.remove(cur);
            if buffer.len() == 1 {
                valid_pages.push(curr_page);
                cur += 1;
            }

            let mut valid: bool = false;
            for i in 0..buffer.len() {
                valid = check_if_valid(curr_page, buffer[i], &rules);
                if !valid {
                    break;
                }
            }
            if valid {
                valid_pages.push(curr_page);
            } else {
                buffer.push(curr_page);
            }
        }
        println!("pages: {:?}", pages);
        println!("solved: {:?}", valid_pages);

        let mid = valid_pages.len() / 2;
        res += valid_pages[mid].parse::<usize>().unwrap();
        all_valid_pages.push(valid_pages);
    }

    return res;
}
