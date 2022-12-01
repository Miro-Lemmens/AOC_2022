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

    let max_cal_elf = elfs
        .iter()
        .max()
        .unwrap();

    println!("{}", max_cal_elf)
}