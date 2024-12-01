use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn get_file_as_string(path: &str)-> String {
    let path = Path::new(path);
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn get_file_as_line_pairs(path: &str) -> (Vec<i32>, Vec<i32>) {
    get_file_as_string(path).lines().map(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        return (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());
    }).unzip()
}


fn day_1_1() -> i32 {
    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = get_file_as_line_pairs("input01_1");

    // println!("{:?}", lefts.first().unwrap());
    // println!("{:?}", rights.first().unwrap());

    lefts.sort();
    rights.sort();

    lefts.iter().zip(rights.iter()).fold(0, |acc, (left, right)| {
        acc + (*right - *left).abs()
    })
}

fn day_1_2() -> i32 {
    let (left, right) = get_file_as_line_pairs("input01_1");

    let mut right_hex_map = HashMap::new();
    right.iter().for_each(|&x| {
        if right_hex_map.contains_key(&x) {
            *right_hex_map.get_mut(&x).unwrap() += 1;
        } else {
            right_hex_map.insert(x, 1);
        }
    });

    left.iter().map(|&x| {
        x * right_hex_map.get(&x).unwrap_or(&0)
    }).sum()
}

fn main() {
    println!("{}", day_1_1());
    println!("{}", day_1_2());
}
