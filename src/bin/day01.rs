use aoc_2025::helpers;

fn main() {
    let input_path = "src/inputs/day1_input.txt";
    let inputs = helpers::get_inputs(input_path);

    part_one(&inputs);
    part_two(&inputs);
}
fn part_one(inputs: &Vec<String>) {
    let mut zero_count = 0;
    let mut dial = 50;
    for input in inputs {
        let direction = input.chars().next().unwrap();
        let steps: u32 = input[1..].parse().unwrap();

        if direction == 'R' {
            dial = (dial + steps) % 100;
        } else if direction == 'L' {
            dial = (dial + 100 - (steps % 100)) % 100;
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    println!("(Part One) Zero count: {}", zero_count);
}

fn part_two(inputs: &Vec<String>) {
    let mut zero_count = 0;
    let mut dial = 50;
    for input in inputs {
        let direction = input.chars().next().unwrap();
        let steps: u32 = input[1..].parse().unwrap();

        if direction == 'R' {
            for _ in 0..steps {
                dial = if dial < 99 { dial + 1 } else { 0 };
                zero_count += (dial == 0) as usize;
            }
        } else if direction == 'L' {
            for _ in 0..steps {
                dial = if dial > 0 { dial - 1 } else { 99 };
                zero_count += (dial == 0) as usize;
            }
        }
    }

    println!("(Part Two) Zero count: {}", zero_count);
}