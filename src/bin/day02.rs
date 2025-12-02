use std::fs;

fn main() {
    let input_path = "src/inputs/day2_input.txt";
    let inputs = fs::read_to_string(input_path).unwrap();

    part_one(&inputs);
    part_two(&inputs);
}

fn part_one(inputs: &String) {
    let ranges = inputs.split(',').collect::<Vec<&str>>();
    let mut invalid_id_sum = 0;

    for range in &ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let mut lower: u64 = bounds[0].parse().unwrap();
        let upper: u64 = bounds[1].parse().unwrap();

        while lower <= upper {
            let lower_str = lower.to_string();
            if lower_str.len() % 2 == 0 {
                let midpoint = lower_str.len() / 2;
                let first_half: u64 = lower_str[..midpoint].parse().unwrap();
                let second_half: u64 = lower_str[midpoint..].parse().unwrap();
                if first_half == second_half {
                    invalid_id_sum += lower;
                }
            }
            lower += 1;
        }
    }

    println!("(Part One) Sum of invalid IDs: {}", invalid_id_sum);
}

fn part_two(inputs: &String) {
    let ranges = inputs.split(',').collect::<Vec<&str>>();
    let mut invalid_id_sum = 0;

    for range in &ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let mut lower: u64 = bounds[0].parse().unwrap();
        let upper: u64 = bounds[1].parse().unwrap();

        while lower <= upper {
            if lower < 10 {
                lower += 1;
                continue;
            };

            let lower_str = lower.to_string();
            let sections = lower_str.len() / 2;

            for section in 1..=sections {
                if lower_str.len() % section != 0 {
                    continue;
                }
                let start = &lower_str[..section];
                let repetitions = start.repeat(lower_str.len() / section);
                if repetitions == lower_str {
                    invalid_id_sum += lower;
                    break;
                }
            }
            lower += 1;
        }
    }

    println!("(Part Two) Sum of invalid IDs: {}", invalid_id_sum);
}
