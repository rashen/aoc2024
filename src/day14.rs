use glam::IVec2;
use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
};

pub fn main() {
    let input = std::fs::read_to_string("input/day14.txt").expect("No input");
    let rows = 103;
    let cols = 101;
    println!("Part 1: {}", part1(&input, rows, cols));
    let _ = part2(&input);
}

#[derive(Debug)]
struct Robot {
    pub pos: IVec2,
    pub vel: IVec2,
}
impl Robot {
    pub fn new(pos: IVec2, vel: IVec2) -> Self {
        Self { pos, vel }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input.lines().filter_map(parse_single).collect()
}

fn parse_single(line: &str) -> Option<Robot> {
    let (p, v) = line.split_once(' ')?;
    let pos = p.split_once('=')?.1.split_once(',')?;
    let pos = IVec2::new(pos.0.parse::<i32>().ok()?, pos.1.parse::<i32>().ok()?);
    let vel = v.split_once('=')?.1.split_once(',')?;
    let vel = IVec2::new(vel.0.parse::<i32>().ok()?, vel.1.parse::<i32>().ok()?);
    Some(Robot::new(pos, vel))
}

fn step(robots: &mut [Robot], rows: usize, cols: usize, steps: i32) {
    let size = IVec2::new(cols as i32, rows as i32);
    for r in robots.iter_mut() {
        r.pos = (r.pos + r.vel * steps).rem_euclid(size);
    }
}

fn safety_factor(robots: &[Robot], rows: usize, cols: usize) -> i32 {
    let cols = cols as i32;
    let rows = rows as i32;
    let mut quadrants = [0; 4];
    // The quadrant order does not matter, as long as we separate them correctly
    for r in robots {
        #[allow(clippy::comparison_chain)]
        if r.pos.x > cols / 2 {
            if r.pos.y > rows / 2 {
                quadrants[0] += 1;
            } else if r.pos.y < rows / 2 {
                quadrants[1] += 1;
            }
        } else if r.pos.x < cols / 2 {
            if r.pos.y > rows / 2 {
                quadrants[2] += 1;
            } else if r.pos.y < rows / 2 {
                quadrants[3] += 1;
            }
        }
    }
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn part1(input: &str, rows: usize, cols: usize) -> i32 {
    let mut robots = parse_input(input);
    for _ in 0..100 {
        step(&mut robots, rows, cols, 1);
    }
    safety_factor(&robots, rows, cols)
}

fn part2(input: &str) -> io::Result<()> {
    let mut robots = parse_input(input);
    let mut steps = 0;
    let rows = 103;
    let cols = 101;

    let mut terminal = ratatui::init();
    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            let ascii_art = create_sprite(&robots, rows, cols);
            let greeting =
                Paragraph::new(format!("Steps: {steps} --- Press q to quit\n{ascii_art}")).red();
            // let info = Paragraph::new("Steps: {steps}").green().on_red();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    break;
                } else if key.code == KeyCode::Char('b') {
                    steps -= 1;
                    step(&mut robots, rows, cols, -1);
                } else {
                    steps += 1;
                    step(&mut robots, rows, cols, 1);
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}

fn pos_to_idx(pos: IVec2, rows: usize, cols: usize) -> Option<usize> {
    if pos.x >= cols as i32 || pos.y >= rows as i32 || pos.x < 0 || pos.y < 0 {
        return None;
    }
    Some((pos.y * (cols as i32) + pos.x) as usize)
}

fn create_sprite(robots: &[Robot], rows: usize, cols: usize) -> String {
    let mut buffer = vec![' '; rows * cols];
    for r in robots {
        let idx = pos_to_idx(r.pos, rows, cols).unwrap();
        buffer[idx] = '#';
    }
    buffer
        .chunks(cols)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let rows = 7;
        let cols = 11;
        assert_eq!(part1(INPUT, rows, cols), 12)
    }

    #[test]
    fn test_step() {
        let rows = 7;
        let cols = 11;
        let mut robots = vec![Robot::new(IVec2::new(2, 4), IVec2::new(2, -3))];
        step(&mut robots, rows, cols, 1);
        assert_eq!(robots[0].pos, IVec2::new(4, 1));
        step(&mut robots, rows, cols, 1);
        assert_eq!(robots[0].pos, IVec2::new(6, 5));
        step(&mut robots, rows, cols, 1);
        step(&mut robots, rows, cols, -1);
        assert_eq!(robots[0].pos, IVec2::new(6, 5));
    }
}
