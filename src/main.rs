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

fn is_increasing_true(vec: &Vec<&str>) -> bool {
    let mut prev = vec.first().unwrap().parse::<i32>().unwrap();
    for num in vec.iter().skip(1) {
        let numa = num.parse::<i32>().unwrap();
        if prev >= numa || (numa - prev).abs() > 3 {

            return false
        }

        prev = numa;
    }

    true
}

fn is_decreasing_true(vec: &Vec<&str>) -> bool {
    let mut prev = vec.first().unwrap().parse::<i32>().unwrap();
    for num in vec.iter().skip(1) {
        let numa = num.parse::<i32>().unwrap();
        if prev <= numa || (numa - prev).abs() > 3 {
            return false
        }
        prev = numa;
    }

    true
}

fn day_2_1() -> i32 {
    get_file_as_string("input02_1").lines().map(|line| {
        let line = line.split(" ").collect::<Vec<&str>>();
        if is_increasing_true(&line) || is_decreasing_true(&line) {
            1
        } else { 0 }
    }).sum()
}

fn is_decreasing_true_damp2(vec: &Vec<&str>) -> bool {
    let mut prev = vec.first().unwrap().parse::<i32>().unwrap();
    let mut true_flag = true;
    for num in vec.iter().skip(1) {
        let numa = num.parse::<i32>().unwrap();
        if prev <= numa || (numa - prev).abs() > 3 {
            true_flag = false;
        }
        prev = numa;
    }

    if !true_flag {

        for (idx, _) in vec.iter().enumerate() {
            let sublist =  [vec.as_slice().split_at(idx).0,
                vec.as_slice().split_at(idx + 1).1].concat();

            if is_decreasing_true(&sublist) {
                true_flag = true;
            }
        }
    }

    true_flag
}

fn is_increasing_true_damp2(vec: &Vec<&str>) -> bool {
    let mut prev = vec.first().unwrap().parse::<i32>().unwrap();
    let mut true_flag = true;
    for num in vec.iter().skip(1) {
        let numa = num.parse::<i32>().unwrap();
        if prev >= numa || (numa - prev).abs() > 3 {
            true_flag = false;
        }
        prev = numa;
    }

    if !true_flag {

        for (idx, _) in vec.iter().enumerate() {
            let sublist =  [vec.as_slice().split_at(idx).0,
                vec.as_slice().split_at(idx + 1).1].concat();

            if is_increasing_true(&sublist) {
                true_flag = true;
            }
        }
    }

    true_flag
}

fn day_2_2() -> i32 {
    get_file_as_string("input02_1").lines().map(|line| {
        let line = line.split(" ").collect::<Vec<&str>>();
        if is_increasing_true_damp2(&line) || is_decreasing_true_damp2(&line) {
            1
        } else { 0 }
    }).sum()
}

fn day_3_1() -> i32 {
    get_file_as_string("input_03").lines().map(|line| {
        line.split("mul(").map(|mul| {
            if let Some((left, right)) = mul.split_once(",") {
                if let Some((right, _)) = right.split_once(")") {
                    return match (left.parse::<i32>(), right.parse::<i32>()) {
                        (Ok(left), Ok(right)) => right * left,
                        _ => 0
                    }
                }
            }

            0
        }).sum::<i32>()
    }).sum()
}

fn day_3_2() -> i32 {
    let binding = get_file_as_string("input_03");
    let lines = binding.split("do()").map(|enable| {
        // println!("\n\n//// line break ////\n\n");

            let split = enable.split("don't()").collect::<Vec<&str>>();
            if let Some(excl) = split.get(1) {
                println!("incl: {} \nexcl: {}\n\n---\n\n", split[0], excl);
            } else {
                println!("{}", split[0]);
            }
            split[0]



    }).collect::<Vec<&str>>();

    for line in binding.lines() {
        // println!("{}\n\n---\n\n", line);
    }

    lines.into_iter().map(|line| {
        line.split("mul(").map(|mul| {
            if let Some((left, right)) = mul.split_once(",") {
                if let Some((right, _)) = right.split_once(")") {
                    return match (left.parse::<i32>(), right.parse::<i32>()) {
                        (Ok(left), Ok(right)) => right * left,
                        _ => 0
                    }
                }
            }

            0
        }).sum::<i32>()
    }).sum()
}

fn horizontal_forward_xmas(file: &String) -> i32 {
    let mut acc = 0;
    let mut state = 0;
    for char in file.chars() {
        match (state, char) {
            (_, 'X') => state = 1,
            (1, 'M') => state += 1,
            (2, 'A') => state += 1,
            (3, 'S') => {
                state = 0;
                acc += 1;
            },
            _ => state = 0,
        }
    }
    acc
}

fn horizontal_backward_xmas(file: &String) -> i32 {
    horizontal_forward_xmas(&file.chars().rev().collect::<String>())
}

fn vertical_downwards_xmas(file: &String) -> i32 {
    let lines = file.lines().collect::<Vec<&str>>();
    let mut acc = 0;
    let mut state = 0;
    for horz_idx in 0..lines[0].len() {
        for vert_idx in 0..lines.len() {
            let char = lines[vert_idx].chars().nth(horz_idx).unwrap();
            match (state, char) {
                (_, 'X') => state = 1,
                (1, 'M') => state += 1,
                (2, 'A') => state += 1,
                (3, 'S') => {
                    state = 0;
                    acc += 1;
                },
                _ => state = 0,
            }
        }
    }
    acc
}

fn vertical_upwards_xmas(file: &String) -> i32 {
    let lines = file.lines().collect::<Vec<&str>>();
    let mut acc = 0;
    let mut state = 0;
    for horz_idx in 0..lines[0].len() {
        for vert_idx in (0..lines.len()).rev() {
            let char = lines[vert_idx].chars().nth(horz_idx).unwrap();
            match (state, char) {
                (_, 'X') => state = 1,
                (1, 'M') => state += 1,
                (2, 'A') => state += 1,
                (3, 'S') => {
                    state = 0;
                    acc += 1;
                },
                _ => state = 0,
            }
        }
    }
    acc
}



fn day_4_1() -> i32 {
    let binding = get_file_as_string("input_04");
    let horizontal_forward = horizontal_forward_xmas(&binding);
    let mut word = [0; 4];
    let map = include_bytes!("../input_04")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();


        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                [
                    [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                    [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                    [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                    [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });
                word.fill_with(|| iter.next().unwrap_or_default());
                &word == b"XMAS" || &word == b"SAMX"
            })
            .count() as i32
}

fn day_4_2() -> i32 {
    let mut cross = [0; 4];
    let map = include_bytes!("../input_04")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();


        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .map(|(x, y)| {
                [
                    (x + 1, y + 1), // Center
                    (x, y),         // NE
                    (x, y + 2),     // SE
                    (x + 2, y),     // NW
                    (x + 2, y + 2), // SW
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });

                if iter.next().is_none_or(|n| n != b'A') {
                    return false;
                }

                cross[0] = iter.next().unwrap_or_default();
                cross[1] = iter.next().unwrap_or_default();
                cross[2] = iter.next().unwrap_or_default();
                cross[3] = iter.next().unwrap_or_default();

                &cross == b"MMSS" || &cross == b"MSMS" || &cross == b"SSMM" || &cross == b"SMSM"
            })
            .count() as i32

}

fn main() {
    // println!("{}", day_1_1());
    // println!("{}", day_1_2());

    // println!("{}", day_2_1());
    // println!("{}", day_2_2());

    // println!("{}", day_3_1());
    // println!("{}", day_3_2());

    // println!("{}", day_4_1());
    // println!("{}", day_4_2());
}

#[cfg(test)]
mod tests {
    use crate::{day_2_1, day_2_2, horizontal_backward_xmas, horizontal_forward_xmas, is_decreasing_true, is_decreasing_true_damp2, is_increasing_true, is_increasing_true_damp2, vertical_downwards_xmas, vertical_upwards_xmas};

    const XMAS_TEST: &str =    "MMMSXXMASM
                                MSAMXMSMSA
                                AMXSXMAAMM
                                MSAMASMSMX
                                XMASAMXAMM
                                XXAMMXXAMA
                                SMSMSASXSS
                                SAXAMASAAA
                                MAMMMXMMMM
                                MXMXAXMASX";

    const XMAS_VERT_TEST: &str =   "SXMMMXMMMM\nAMMMMXMMSM\nMAMMMMMMAX\nXSMMMAMMMM\nMMMMMSMMXS";

    #[test]
    fn test_forward_search() {
        assert_eq!(horizontal_forward_xmas(&XMAS_TEST.into()), 3);
    }

    #[test]
    fn test_backward_search() {
        assert_eq!(horizontal_backward_xmas(&XMAS_TEST.into()), 2);
    }

    #[test]
    fn test_vert_down_search() {
        assert_eq!(vertical_downwards_xmas(&XMAS_VERT_TEST.into()), 2);
    }

    #[test]
    fn test_vert_up_search() {
        assert_eq!(vertical_upwards_xmas(&XMAS_VERT_TEST.into()), 2);
    }

    #[test]
    fn test_is_decreasing_true() {
        assert_eq!(is_decreasing_true(&vec!["19", "16", "14", "13", "10", "9"]), true);
        assert_eq!(is_decreasing_true(&vec!["19", "15", "14", "13", "10", "9"]), false);
    }

    #[test]
    fn test_is_increasing_true() {
        assert_eq!(is_increasing_true(&vec!["9", "10", "12", "15", "16", "19"]), true);
        assert_eq!(is_increasing_true(&vec!["9", "10", "12", "15", "16", "20"]), false);
    }

    #[test]
    fn test_is_decreasing_true_damp() {
        assert_eq!(is_decreasing_true_damp2(&vec!["7", "6", "4", "2", "1"]), true);
        assert_eq!(is_decreasing_true_damp2(&"9 7 6 2 1".split(" ").collect()), false);
        assert_eq!(is_decreasing_true_damp2(&"8 6 4 4 1".split(" ").collect()), true);
        assert_eq!(is_decreasing_true_damp2(&"7 6 6 5 3".split(" ").collect()), true);
    }

    #[test]
    fn test_is_increasing_true_damp2() {
        assert_eq!(is_increasing_true_damp2(&"1 2 7 8 9".split(" ").collect()), false);
        assert_eq!(is_increasing_true_damp2(&"1 3 2 4 5".split(" ").collect()), true);
        assert_eq!(is_increasing_true_damp2(&"1 3 6 7 9".split(" ").collect()), true);
    }

    #[test]
    fn test_days_2() {
        assert_eq!(day_2_1(), 257);
        assert_eq!(day_2_2(), 328);
    }
}
