pub fn main() {
    let input = std::fs::read_to_string("input/day9.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Block {
    pub data: Vec<usize>,
    pub size: usize,
}
impl Block {
    pub fn free(&self) -> usize {
        self.size - self.data.len()
    }
}

#[allow(unused)]
fn parse_memory(memory: &[Block]) -> String {
    let mut output = String::new();
    for m in memory {
        for d in &m.data {
            output += &d.to_string();
        }
        output += &".".repeat(m.free());
    }
    output
}

fn part1(input: &str) -> i64 {
    let mut memory = Vec::new();

    let mut occupied = true;
    let mut id = 0;
    for c in input.chars() {
        if let Some(size) = c.to_digit(10) {
            let size = size as usize;
            if occupied {
                memory.push(Block {
                    data: vec![id; size],
                    size,
                });
                id += 1;
            } else {
                memory.push(Block { data: vec![], size });
            }
            occupied = !occupied;
        }
    }
    let mut forward = 0;
    let mut backward = memory.len() - 1;
    while forward < backward {
        while memory[forward].free() == 0 {
            forward += 1;
        }
        while memory[backward].data.len() == 0 {
            backward -= 1;
        }

        while memory[forward].free() > 0 && memory[backward].data.len() > 0 {
            let to_move = memory[backward].data.pop().unwrap();
            memory[forward].data.push(to_move);
        }
        if memory[forward].free() == 0 {
            forward += 1;
        }
        if memory[backward].data.len() == 0 {
            backward -= 1;
        }
    }

    memory
        .iter()
        .map(|m| &m.data)
        .flatten()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + i * d) as i64
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }
}
