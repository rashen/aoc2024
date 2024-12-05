#!/usr/bin/bash

if [[ $# -gt 0 ]]; then
    NEW_DAY=$1
else
    LAST_DAY="$(ls src | grep day | cut -c 4 | sort | tail -1)"
    NEW_DAY=$(($LAST_DAY + 1))
fi

FILE="src/day${NEW_DAY}.rs"

if [[ -f $FILE ]]; then
    echo "File already exists"
    exit 1
fi

echo "Creating $FILE"
touch $FILE
cat >$FILE <<EOL
pub fn main() {
    let input = std::fs::read_to_string("input/day${NEW_DAY}.txt").expect("No input");
    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
EOL

fetch_input.sh ${NEW_DAY}
