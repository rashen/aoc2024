use std::env;

mod day1;

fn main() {
    let entry_points = [day1::main];

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
