use lib::read_lines;
use std::mem::replace;

fn main() {
    let lines = read_lines();
    let mut gamma = Vec::<Vec::<i32>>::new();
    let mut epsilon = Vec::<Vec::<i32>>::new();
    for line in lines {
        let mut row = Vec::<char>::new();
        for char in line.chars() {
            row.push(char);
        }
        let pushrow = row
            .iter()
            .map(|c| *c as i32)
            .map(|c| { c - 48 })
            .collect::<Vec::<i32>>();

        gamma.push(pushrow.clone());

        for mut num in 0..pushrow.len() {
            if num == 0 {
                let _ = replace(&mut num, 1);
            } else {
                let _ = replace(&mut num, 0);
            }
        }
        epsilon.push(pushrow.clone());
    }

    let gamma_f = create_2d_vec(gamma);
    let gamma_b = gamma_f
        .iter()
        .map(|c| amt_to_bin(*c))
        .collect::<Vec<i32>>();

    let epsilon_f = create_2d_vec(epsilon);
    let epsilon_b = epsilon_f
        .iter()
        .map(|c| amt_to_bin(*c))
        .collect::<Vec<i32>>();

    println!("{}", bin_vec_to_dec(gamma_b) * bin_vec_to_dec(epsilon_b));

}

fn amt_to_bin(num: i32) -> i32 {
    let k = (1000 / num)-1;
    return k
}

fn bin_vec_to_dec(vector: Vec<i32>) -> i32 {
    let mut r = 0;
    for i in 0..vector.len() {
        r += 2_i32.pow(i as u32) * vector[i];
    }
    return r
}

fn create_2d_vec(vector: Vec::<Vec::<i32>>) -> Vec::<i32> {
    let mut f = <Vec::<i32>>::new();
    for i in 0..vector[0].len() {
        let mut gamma_temp = 0;
        for row in vector.clone() {
            gamma_temp += row[i];
        }
        f.push(gamma_temp);
    }
    return f
}