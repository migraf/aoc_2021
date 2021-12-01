use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day_1()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_1() {
    let lines = read_lines("input/input_1.txt").unwrap();

    let scans = lines
        .map(|line| line
            .unwrap()
            .parse::<i32>()
            .unwrap()
        )
        .collect::<Vec<i32>>();

    let mut count = 0;
    let mut previous = scans[0];
    for scan in &scans {
        if scan > &previous {
            count += 1;
        }
        previous = scan.clone();
    }
    println!("Task 1: {}", count);
    previous = scans.windows(3)
        .nth(0)
        .unwrap()
        .iter()
        .sum();
    let mut window_count = 0;
    for window in scans.windows(3) {
        let sum = window.iter().sum::<i32>();
        if sum > previous {
            window_count += 1;
        }
        previous = sum;
    }
    println!("Task 2: {}", window_count);
}
