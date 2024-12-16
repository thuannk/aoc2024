use std::env;
use std::error::Error;
use std::time::Instant;
use std::collections::HashMap;
use std::fs;

pub mod d1p1;
pub mod d1p2;

fn timer(data: &String, func: &fn(&String) -> ()) -> Result<(), Box<dyn Error>>
{
    let start = Instant::now();
    func(data);
    let end = Instant::now();

    println!("Duration: {:?}", end.duration_since(start));
    return Ok(());
}

fn main() {
    let mut problems_map: HashMap<&str, fn(&String) -> ()> = HashMap::new();
    problems_map.insert("1-1", d1p1::solution);
    problems_map.insert("1-2", d1p2::solution);

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: aoc2024 <day>-<problem> <input_file>. Eg: aoc2024 1-1 ./input");
        return;
    }

    let solution = problems_map.get(args[1].as_str()).expect("No solution found");
    let data = fs::read_to_string(args[2].as_str()).expect("Can't read input file");

    let result = timer(&data, solution);
    println!("Result: {:#?}", result);
}
