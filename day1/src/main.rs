use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn part1() -> i32 {
    let mut data = String::new();
    let mut smallest_left: Vec<i32> = Vec::new();
    let mut smallest_right: Vec<i32> = Vec::new();
    let mut f = fs::File::open("day1/src/input.txt").unwrap();
    f.read_to_string(&mut data).unwrap();
    for  line in data.split('\n') {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            smallest_left.push(a.parse::<i32>().unwrap());
            smallest_right.push(b.parse::<i32>().unwrap());
        }
    }
    smallest_left.sort();
    smallest_right.sort();
    assert_eq!(smallest_left.len(), smallest_right.len());
    let result: Vec<_>= smallest_left
        .iter()
        .zip(smallest_right.iter())
        .map(|(&a, &b)| (a - b).abs())
        .collect();
    result.iter().sum::<i32>()
}
fn part2() -> usize {
    let mut data = String::new();
    let mut smallest_left: Vec<usize> = Vec::new();
    let mut smallest_right: Vec<usize> = Vec::new();
    let mut f = fs::File::open("day1/src/input.txt").unwrap();
    f.read_to_string(&mut data).unwrap();
    for line in data.split('\n') {
        let mut parts = line.split_whitespace();
        if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
            smallest_left.push(a.parse::<usize>().unwrap());
            smallest_right.push(b.parse::<usize>().unwrap());
        }
    }
    smallest_left.sort();
    smallest_right.sort();
    let mut appears_count: HashMap<usize, usize> = HashMap::new();
    for i in smallest_left.iter() {
        let count = smallest_right.iter().filter(|&x| *x == *i).count();
        appears_count.insert(*i, count);
    }
    appears_count.iter().map(|(a,b)| a*b).sum::<usize>()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}