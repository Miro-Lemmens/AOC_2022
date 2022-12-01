use lib::read_lines_as_i32;

fn main() {
    let lines = read_lines_as_i32();
    let mut r = 0;
    for i in 3..lines.len() {
        if lines[i - 1] + lines[i - 2] + lines[i - 3] < lines[i] + lines[i - 1] + lines[i - 2] {
            r += 1
        }
    }
    println!("{r}");
}