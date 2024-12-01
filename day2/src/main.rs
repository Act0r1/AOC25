use std::collections::HashMap;
use std::fs;
use std::io::Read;

fn main() {
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
    let result = appears_count.iter().map(|(a,b)| a*b).sum::<usize>();
    println!("Part: {}", result);
}
