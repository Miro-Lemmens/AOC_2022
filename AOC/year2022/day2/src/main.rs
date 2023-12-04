use lib::read_lines;

fn get_rps(line: String) -> [char; 2] {
    let (x, y) = (line.as_bytes()[0] as char, line.as_bytes()[2] as char);
    return [x,y]
}

fn convert(plays: Vec::<[char;2]>) -> Vec::<Vec<i32>> {
    let mut plays_f = Vec::<Vec::<i32>>::new();
    for i in 0..plays.len() {
        plays_f.push(vec![]);
        if plays[i][0] == 'A' {
            plays_f[i].push(1);
        } else if plays[i][0] == 'B' {
            plays_f[i].push(2);
        } else if plays[i][0] == 'C' {
            plays_f[i].push(3);
        }

        if plays[i][1] == 'X' {
            plays_f[i].push(1);
        } else if plays[i][1] == 'Y' {
            plays_f[i].push(2);
        } else if plays[i][1] == 'Z' {
            plays_f[i].push(3);
        }
    }
    return plays_f
}

fn part1(plays_f: Vec::<Vec::<i32>>, len: usize) -> i32 {
    let mut r = 0;
    for i in 0..len {
        if plays_f[i][0] == plays_f[i][1] {
            r += 3_i32;
        } else if plays_f[i][0] == 3 && plays_f[i][1] == 1 {
            r += 6_i32;
        } else if plays_f[i][0] == 2 && plays_f[i][1] == 3 {
            r += 6_i32;
        } else if plays_f[i][0] == 1 && plays_f[i][1] == 2 {
            r += 6_i32;
        }
        r += plays_f[i][1] as i32;
    }
    return r
}

fn part2(plays_f: Vec::<Vec::<i32>>, len: usize) -> i32 {
    let mut r = 0;
    for i in 0..len {
        if plays_f[i][1] == 2 {
            r += 3_i32;
            if plays_f[i][0] == 1 {
                r += 1_i32;
            } else if plays_f[i][0] == 2 {
                r += 2_i32;
            } else {
                r += 3_i32;
            }
        } else if plays_f[i][1] == 3 {
            r += 6_i32;
            if plays_f[i][0] == 1 {
                r += 2_i32;
            } else if plays_f[i][0] == 2 {
                r += 3_i32;
            } else {
                r += 1_i32;
            }
        } else {
            if plays_f[i][0] == 1 {
                r += 3_i32;
            } else if plays_f[i][0] == 2 {
                r += 1_i32;
            } else {
                r += 2_i32;
            }
        }
    }
    return r
}

fn main() {
    let lines = read_lines();
    let plays = lines
        .iter()
        .map(|c| get_rps(c.to_string()))
        .collect::<Vec<[char; 2]>>();

    let plays_f = convert(plays);
    println!("{}", part1(plays_f.clone(), plays_f.len().clone()));
    println!("{}", part2(plays_f.clone(), plays_f.len().clone()))
}

