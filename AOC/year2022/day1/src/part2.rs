use lib::read_lines;

fn main() {
    let lines = read_lines();
    let mut elfs = Vec::new();
    let mut elf  = 0;
    for cal in lines {
        if cal.is_empty() {
            elfs.push(elf);
            elf = 0;
        } else {
            elf += cal
                .parse::<i32>()
                .unwrap();
        }
    }
    elfs.sort();
    let total_cal = elfs[elfs.len() - 1] + elfs[elfs.len() - 2] + elfs[elfs.len() - 3];

    println!("{}", total_cal);
}