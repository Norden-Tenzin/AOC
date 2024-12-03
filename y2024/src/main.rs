mod day1;
mod day2;
mod day3;

fn main() {
    // Day 1
    let res = day1::problem1("inputs/day1.txt");
    println!("{}", res);
    day1::problem2("inputs/day1.txt");

    // Day 2
    let res = day2::problem1("inputs/day2.txt");
    println!("{}", res);
    let res = day2::problem2("inputs/day2.txt");
    println!("{}", res);

    // Day 3
    let res = day3::problem1("inputs/day3.txt");
    println!("{}", res);
    day3::problem2("inputs/day3.txt");
}
