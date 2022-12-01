use std::fs::read_to_string;

pub fn read_string() -> String {
    let path = format!("src/input.txt");
    return read_to_string(path)
        .expect("Couldn't read file")
        .trim().to_string();
}

pub fn read_lines() -> Vec<String> {
    return read_string()
        .lines()
        .map(| l | l.trim().to_string())
        .collect();
}

pub fn read_lines_as_i32() -> Vec<i32> {
    return read_lines()
        .into_iter()
        .map(| l | l.parse::<i32>().unwrap())
        .collect()
}

pub fn read_split_on_empty_line() -> Vec<String> {
    return read_string()
        .split("\n\n")
        .map(| l | l.trim().to_string())
        .collect();
}