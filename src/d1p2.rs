use std::collections::HashMap;

pub fn solution(data: &String) {
    let (left, right) = data.lines().map(|line | {
        let (l, r) = line.split_once(|c: char| {c.is_whitespace()}).unwrap();
        (l.parse::<i32>().unwrap(), r.trim().parse::<i32>().unwrap())
    }).collect::<(Vec<i32>, Vec<i32>)>();

    let right_accumulated_map = right.iter().fold(HashMap::new(), |mut acc, &e| {
        if acc.contains_key(&e) {
            acc.insert(e, acc.get(&e).unwrap() + 1);
        } else {
            acc.insert(e, 1);
        }
        acc
    });

    let result = left.iter().fold(0, |acc, e| {
        acc + e * right_accumulated_map.get(&e).unwrap_or(&0)
    });

    println!("{result}");
}