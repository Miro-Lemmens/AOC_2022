use lib::read_lines;

fn main() {
    let lines = read_lines();
    let mut x = 0;
    let mut y = 0;
    let mut aim  = 0;
    for line in lines {
        if line.starts_with("forward") {
            x += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            y += aim * line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        } else if line.starts_with("up") {
            aim -= line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        } else {
            aim += line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", x*y);
}
