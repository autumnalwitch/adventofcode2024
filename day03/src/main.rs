use std::fs::File;
use std::io::Read;

use regex::Regex;

fn read_input() -> String {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

fn main() {
    let contents = read_input();
    println!("{}", contents);
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut result = 0;
    let mut doing = true;
    for capture in re.captures_iter(&contents) {
        println!("{:?}", &capture);
        if capture.get(4).is_some() {
            println!("do");
            doing = true;
        } else if capture.get(5).is_some() {
            println!("don't");
            doing = false;
        } else if doing {
            let a: i64 = capture[2].parse().unwrap();
            let b: i64 = capture[3].parse().unwrap();
            println!("{}, {}", a, b);
            result += a * b;
        }
    }
    println!("{}", result);
}
