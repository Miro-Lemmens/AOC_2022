use lib::read_lines_as_i32;

fn main() {
    let lines = read_lines_as_i32();
    let mut r = 0;
    for i in 1..lines.len() {
        if lines[i - 1] < lines[i] {
            r += 1
        }
    }
    println!("{r}");
}

