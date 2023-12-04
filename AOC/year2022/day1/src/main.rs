use lib::read_lines;

fn create_elfs_list(lines: Vec::<String>) -> Vec::<i32> {
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
    return elfs
}

fn part1(elfs: Vec::<i32>) -> i32 {
    let max_cal_elf = elfs
        .iter()
        .max()
        .unwrap();

    return *max_cal_elf
}

fn part2(mut elfs: Vec::<i32>) -> i32 {
    elfs.sort();
    let total_cal = elfs[elfs.len() - 1] + elfs[elfs.len() - 2] + elfs[elfs.len() - 3];
    return total_cal
}

fn main() {
    let lines = read_lines();
    let elfs_list = create_elfs_list(lines);
    println!("{}", part1(elfs_list.clone()));
    println!("{}", part2(elfs_list.clone()));
}