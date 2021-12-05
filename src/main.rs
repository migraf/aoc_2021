use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::SplitWhitespace;

fn main() {
    day_3()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_3() {
    let lines: Vec<String> = read_lines("input/day_3.txt")
        .unwrap()
        .map(|line| line.unwrap())
        .collect();


    #[derive(Debug)]
    struct DiagnosticReport {
        diagnostics: Vec<String>,
        gamma_rate: i32,
        epsilon_rate: i32,
    }

    impl DiagnosticReport {
        fn one_counts(&self) -> Vec<i32> {
            let mut one_counts = vec![0; 12];
            for diagnostic in &self.diagnostics {
                let numbers: Vec<i32> = diagnostic
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect();
                for (index, number) in numbers.iter().enumerate() {
                    if *number == 1 {
                        one_counts[index] += 1;
                    }
                }
            }
            one_counts
        }
        fn power_consumption(&self) -> i32 {
            let one_counts = self.one_counts();
            let total = &self.diagnostics.len() / 2;

            // if the one count is greater than the total, set a binary 1 for the gamma rate
            let gamma_rate_binary = one_counts
                .iter()
                .map(|count| if *count > total as i32 { 1 } else { 0 })
                .collect::<Vec<i32>>();


            let epsilon_rate_binary = gamma_rate_binary
                .iter()
                .map(|bit| if *bit == 1 { 0 } else { 1 })
                .collect::<Vec<i32>>();


            let gamma_string = gamma_rate_binary
                .iter()
                .map(|bit| bit.to_string())
                .collect::<Vec<String>>()
                .join("");

            let epsilon_string = epsilon_rate_binary.
                iter()
                .map(|bit| bit.to_string())
                .collect::<Vec<String>>()
                .join("");

            let gamma_rate = i32::from_str_radix(&gamma_string, 2).unwrap();
            let epsilon_rate = i32::from_str_radix(&epsilon_string, 2).unwrap();

            println!("Gamma {:?}, Epsilon {:?}", gamma_rate, epsilon_rate);
            return gamma_rate * epsilon_rate;
        }

        fn life_support_rating(&self) -> i32 {
            let one_counts = self.one_counts();
            let total = &self.diagnostics.len() / 2;
            let mut oxygen_rating: Vec<String> = Vec::new();
            let mut co2_rating: Vec<String> = Vec::new();
            for diagnostic in &self.diagnostics {
                let mut add_oxygen: bool = true;
                let mut add_co2: bool = true;
                for one_count in &one_counts {
                    if *one_count > total as i32 {
                        add_co2 = false;
                    } else {
                        add_oxygen = false;
                    }
                }
                if add_oxygen {
                    oxygen_rating.push(diagnostic.clone());
                }
                if add_co2 {
                    co2_rating.push(diagnostic.clone());
                }
            }

            println!("Oxygen {:?}, CO2 {:?}", oxygen_rating, co2_rating);



            0
        }
    }

    let mut diagnostic_report = DiagnosticReport {
        diagnostics: lines,
        gamma_rate: 0,
        epsilon_rate: 0,
    };
    let power_consumption = diagnostic_report.power_consumption();
    let life_support = diagnostic_report.life_support_rating();

    println!("Power consumption: {:?}", power_consumption);
    println!("Life support {:?}", life_support);
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
            }
            "down" => {
                let distance: i32 = split[1].parse().unwrap();
                y -= distance;
                aim += distance;
            }
            "up" => {
                let distance: i32 = split[1].parse().unwrap();
                y += distance;
                aim -= distance;
            }
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
