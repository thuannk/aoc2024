pub fn solution(data: &String) {
    let (mut left, mut right) = data.lines().map(|line | {
        let (l, r) = line.split_once(|c: char| {c.is_whitespace()}).unwrap();
        (l.parse::<i32>().unwrap(), r.trim().parse::<i32>().unwrap())
    }).collect::<(Vec<i32>, Vec<i32>)>();

    left.sort();
    right.sort();

    let mut result = 0;
    left.iter().zip(right.iter()).for_each(|(l, r)| {
        result += (l - r).abs();
    });

    println!("{result}");
}