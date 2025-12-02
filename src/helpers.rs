use std::fs;

pub fn get_inputs(input_path: &str) -> Vec<String> {
    let input_content = fs::read_to_string(input_path).unwrap();
    let inputs: Vec<String> = input_content.lines().map(|line| line.to_string()).collect();
    inputs
}
