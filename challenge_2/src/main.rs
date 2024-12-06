use std::env;
use std::fs;
use std::str::FromStr;

/**
* Thoughts 
* Gotta process the input which is simple: just newline split and whitespace split
* All increasing or all decreasing is just the first two and then check further also not bad
* Then it's as simple as just taking two at a time and assuring the invarient incrementing a count
* if safe or not.
* This is straightforwardly linear + constant space. 
*/

fn main() {
    // Open file
    // Process into a vec or vecs like [[1,3,3,4,5], [1,2,3,4,151], ...]
    // Safety check 
    // add to a sum or not
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reports = read_input(file_path);
    let report_count = reports.len();
    println!("Hello, I read these reports {file_path}");
    println!("num reports: {report_count}");
    let mut safe_report_count = 0;
    for report in reports {
        if check_safety(&report) {
            safe_report_count += 1;
        }
    }
    println!("safe report count: {safe_report_count}");
}

fn read_input(filepath: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(filepath).expect("THERE'S NO FILE THERE DUMMY");
    let newline_split: Vec<&str> = contents.split("\n").collect();
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in newline_split {
        reports.push(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())  // Use closure with explicit parse method
                .collect()
        );
    }
    return reports;
}

fn check_safety(report: &Vec<i32>) -> bool {
    // need some input
    if report.len() == 0 {
        return false;
    }
    let first = report[0];
    let second = report[1];

    if first == second { 
        return false;
    }

    let increasing = first < second;
    for vals in report.windows(2) {
        let prev = vals[0];
        let next = vals[1];
        // Check the increasing cond
        if prev == next || increasing != (prev < next) {
            return false;
        }

        // Check the dist.
        let dist = (prev - next).abs();
        if dist > 3 {
            return false;
        }
    }
    return true;
}
