#![allow(unstable_name_collisions)]

use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

fn main() {
    let entry_points = [
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day17::main,
    ];

    let args: Vec<String> = env::args().collect();
    let mut index = entry_points.len() - 1;

    if args.len() > 1 {
        if let Ok(day) = args[1].as_str().parse::<usize>() {
            index = day - 1;
        }
    };

    println!("Running day {}:", index + 1);

    entry_points[index]()
}
