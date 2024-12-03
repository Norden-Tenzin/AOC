use std::fs;

pub fn problem1(path: &str) -> i32 {
    fn extract_pattern(input: &str) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == 'm' && chars.peek() == Some(&'u') {
                chars.next();
                if chars.peek() == Some(&'l') {
                    chars.next();
                    if chars.peek() == Some(&'(') {
                        chars.next();
                        // Extract the content inside parentheses
                        let mut content = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch == ')' {
                                chars.next(); // Consume ')'
                                res.push(format!("mul({})", content));
                                break;
                            } else if ch.is_ascii_digit() || ch == ',' {
                                content.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        return res;
    }

    let file = fs::read_to_string(path).expect("unable to read file.");

    let mut res: i32 = 0;

    for line in file.lines() {
        let line_patterns: Vec<String> = extract_pattern(line);
        line_patterns.iter().for_each(|pattern| {
            let inside = &pattern[4..pattern.len() - 1];
            let nums: Vec<i32> = inside
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            res += nums[0] * nums[1];
        });
    }
    return res;
}

pub fn problem2(path: &str) {
    fn extract_pattern(input: &str, found_do: &mut bool, found_dont: &mut bool) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == 'd' && chars.peek() == Some(&'o') {
                chars.next();
                if chars.peek() == Some(&'(') {
                    chars.next();
                    if chars.peek() == Some(&')') {
                        chars.next();
                        *found_do = true;
                        *found_dont = false;
                    }
                } else if chars.peek() == Some(&'n') {
                    chars.next();
                    if chars.peek() == Some(&'\'') {
                        chars.next();
                        if chars.peek() == Some(&'t') {
                            chars.next();
                            if chars.peek() == Some(&'(') {
                                chars.next();
                                if chars.peek() == Some(&')') {
                                    chars.next();
                                    *found_do = false;
                                    *found_dont = true;
                                }
                            }
                        }
                    }
                }
            }
            if c == 'm' && chars.peek() == Some(&'u') {
                chars.next();
                if chars.peek() == Some(&'l') {
                    chars.next();
                    if chars.peek() == Some(&'(') {
                        chars.next();
                        // Extract the content inside parentheses
                        let mut content = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch == ')' {
                                chars.next(); // Consume ')'
                                if *found_do || (!*found_dont && !*found_do) {
                                    res.push(format!("mul({})", content));
                                }
                                break;
                            } else if ch.is_ascii_digit() || ch == ',' {
                                content.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        return res;
    }

    let file = fs::read_to_string(path).expect("unable to read file.");

    let mut res: i32 = 0;
    let mut found_do = false;
    let mut found_dont = false;

    for line in file.lines() {
        let line_patterns: Vec<String> = extract_pattern(line, &mut found_do, &mut found_dont);
        line_patterns.iter().for_each(|pattern| {
            let inside = &pattern[4..pattern.len() - 1];
            let nums: Vec<i32> = inside
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            //println!("{} * {} -> {:?}", nums[0], nums[1], nums[0] * nums[1]);
            res += nums[0] * nums[1];
        });
    }
    println!("{}", res);
}
