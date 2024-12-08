use std::fs;

const FILE_NAME: &str = "day2/src/input.txt";

fn main() {
    let file: String = read_file(FILE_NAME);
    let mut count = 0;
    for f in file.lines() {
        let line = f
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut ok: bool = true;
        let mut only_inc = true;
        let mut only_dec = true;

        for i in 0..line.len() - 1 {
            let diff = line[i + 1] - line[i];
            if diff > 0 {
                only_dec = false;
            } else if diff < 0 {
                only_inc = false;
            }
            if !(1 <= diff.abs() && diff.abs() <= 3) {
                ok = false;
                break;
            }
        }
        if ok && (only_inc || only_dec) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
