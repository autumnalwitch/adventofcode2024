use std::fs::File;
use std::io::Read;

fn read_input() -> String {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}

fn main() {
    // let contents = read_input();
    // println!("{}", contents);
    // let a: Vec<Vec<_>> = contents
    //     .lines()
    //     .map(|line| line.chars().collect())
    //     .collect();
    // let mut sum = 0;
    // for i in 0..a.len() {
    //     for j in 0..a[i].len() - 3 {
    //         if a[i][j] == 'X' && a[i][j + 1] == 'M' && a[i][j + 2] == 'A' && a[i][j + 3] == 'S' {
    //             sum += 1;
    //         }
    //         if a[i][j] == 'S' && a[i][j + 1] == 'A' && a[i][j + 2] == 'M' && a[i][j + 3] == 'X' {
    //             sum += 1
    //         }
    //     }
    // }
    // for i in 0..a.len() - 3 {
    //     for j in 0..a[i].len() {
    //         if a[i][j] == 'X' && a[i + 1][j] == 'M' && a[i + 2][j] == 'A' && a[i + 3][j] == 'S' {
    //             sum += 1;
    //         }
    //         if a[i][j] == 'S' && a[i + 1][j] == 'A' && a[i + 2][j] == 'M' && a[i + 3][j] == 'X' {
    //             sum += 1
    //         }
    //     }
    // }

    // for i in 0..a.len() - 3 {
    //     for j in 0..a[i].len() - 3 {
    //         if a[i][j] == 'X'
    //             && a[i + 1][j + 1] == 'M'
    //             && a[i + 2][j + 2] == 'A'
    //             && a[i + 3][j + 3] == 'S'
    //         {
    //             sum += 1;
    //         }
    //         if a[i][j] == 'S'
    //             && a[i + 1][j + 1] == 'A'
    //             && a[i + 2][j + 2] == 'M'
    //             && a[i + 3][j + 3] == 'X'
    //         {
    //             sum += 1
    //         }
    //     }
    // }

    // for i in 0..a.len() - 3 {
    //     for j in 0..a[i].len() - 3 {
    //         if a[i + 3][j] == 'X'
    //             && a[i + 2][j + 1] == 'M'
    //             && a[i + 1][j + 2] == 'A'
    //             && a[i][j + 3] == 'S'
    //         {
    //             sum += 1;
    //         }
    //         if a[i + 3][j] == 'S'
    //             && a[i + 2][j + 1] == 'A'
    //             && a[i + 1][j + 2] == 'M'
    //             && a[i][j + 3] == 'X'
    //         {
    //             sum += 1
    //         }
    //     }
    // }
    // println!("{sum}");

    let contents = read_input();
    println!("{}", contents);
    let a: Vec<Vec<_>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut sum = 0;
    for i in 0..a.len() - 2 {
        for j in 0..a[i].len() - 2 {
            if ((a[i][j] == 'M'
                && a[i+2][j + 2] == 'S' || a[i][j] == 'S' && a[i+2][j+2] == 'M')
                && a[i + 1][j + 1] == 'A') && ((a[i+2][j] == 'S' && a[i][j+2] == 'M' || a[i+2][j] == 'M' && a[i][j+2] == 'S') && a[i+1][j+1] == 'A')
            {
                sum += 1;
            }
        }
    }
    println!("{sum}");
}

