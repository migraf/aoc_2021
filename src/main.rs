use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::SplitWhitespace;

fn main() {
    day_2()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_2() {
    let lines = read_lines("input/day_2.txt").unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let commands: Vec<String> = lines.map(|line| line.unwrap()).collect();
    for command in commands {
        let split: Vec<&str> = command.split_whitespace().collect();
        match split[0] {
            "forward" => {
                let distance: i32 = split[1].parse().unwrap();
                x += distance;
                x2 += distance;
                y2 += aim * distance;
            },
            "down" => {
                let distance: i32 = split[1].parse().unwrap();
                y -= distance;
                aim += distance;
            },
            "up" => {
                let distance: i32 = split[1].parse().unwrap();
                y += distance;
                aim -= distance;
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
    println!("Task 1: {}", x.abs() * y.abs());
    println!("Task 2: {}", x2.abs() * y2.abs());

}

fn day_1() {
    let lines = read_lines("input/day_1.txt").unwrap();

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

    // Part 2
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
