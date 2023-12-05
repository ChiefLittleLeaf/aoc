use md5;
use std::fs;
use std::time::Instant;

fn main() {
    let file = "/home/m0h02h4/rust/aoc/resources/day4.input";
    //let file = "pqrstuv";
    //let file = "abcdef";
    let input_raw: String = fs::read_to_string(file).unwrap();

    let start = Instant::now();

    let mut i: u64 = 0;
    let mut part1: u64 = 0;
    let part2: u64;

    'outer: loop {
        i += 1;
        let hash = md5::compute((input_raw.trim().to_string() + &i.to_string()).as_bytes());
        //let hash = md5::compute((file.to_string() + &i.to_string()).as_bytes());

        for j in 0..=1 {
            if hash[j] != 0 {
                continue 'outer;
            };
        }
        if hash[2] < 16 {
            if part1 == 0 {
                part1 = i;
            }
            if hash[2] == 0 {
                part2 = i;
                break;
            }
        }
    }
    let end = start.elapsed().as_micros();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Time: {} μs", end);
}
