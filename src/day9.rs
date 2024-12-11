pub fn main() {
    let input = std::fs::read_to_string("input/day9.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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
fn memory_to_string(memory: &[Block]) -> String {
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
    let mut memory = read_input(input);
    let mut forward = 0;
    let mut backward = memory.len() - 1;
    while forward < backward {
        while memory[forward].free() == 0 {
            forward += 1;
        }
        while memory[backward].data.is_empty() {
            backward -= 1;
        }

        while memory[forward].free() > 0 && !memory[backward].data.is_empty() {
            let to_move = memory[backward].data.pop().unwrap();
            memory[forward].data.push(to_move);
        }
        if memory[forward].free() == 0 {
            forward += 1;
        }
        if memory[backward].data.is_empty() {
            backward -= 1;
        }
    }

    memory
        .iter()
        .flat_map(|m| &m.data)
        .enumerate()
        .fold(0, |acc, (i, d)| acc + i * d) as i64
}

fn read_input(input: &str) -> Vec<Block> {
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
    memory
}

fn part2(input: &str) -> i64 {
    let mut memory = read_input(input);

    let mut backward = memory.len() - 1;
    while backward > 0 {
        while memory[backward].data.is_empty() {
            backward -= 1;
        }

        for i in 0..backward {
            if memory[i].free() > 0 && memory[i].free() >= memory[backward].data.len() {
                while let Some(v) = memory[backward].data.pop() {
                    memory[i].data.push(v);
                }
                break;
            }
        }
        backward -= 1;
    }

    memory
        .iter_mut()
        .flat_map(|m| {
            // Zero-padding because we are not perfectly packed like in part 1
            m.data.append(&mut vec![0; m.free()]);
            &m.data
        })
        .enumerate()
        .fold(0, |acc, (i, d)| acc + i * d) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
