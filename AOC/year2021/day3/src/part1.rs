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
        let mut pushrow = row
            .iter()
            .map(|c| *c as i32)
            .map(|c| { c - 48 })
            .collect::<Vec::<i32>>();

        gamma.push(pushrow.clone());

        for num in 0..pushrow.len() {
            if num == 0 {
                let _ = replace(&mut pushrow[num], 1);
            } else {
                let _ = replace(&mut pushrow[num], 0);
            }
        }
        epsilon.push(pushrow.clone());
    }

    let (gamma_f, epsilon_f) = two_to_one_dimension(gamma.clone());

    println!("{:?} {:?}", bin_to_dec(gamma_f.clone()), bin_to_dec(epsilon_f.clone()));
    println!("{}", bin_to_dec(gamma_f.clone())* bin_to_dec(epsilon_f.clone()));
}

fn bin_to_dec(vector: Vec<i32>) -> i32 {
    let mut r = 0;q
    for i in 1..(vector.len()+1) {
        r += 2_i32.pow(i as u32 - 1_u32) * vector[vector.len()-i];
    }
    return r
}

fn two_to_one_dimension(vector: Vec::<Vec::<i32>>) -> (Vec::<i32>, Vec::<i32>) {
    let mut g = <Vec::<i32>>::new();
    let mut e = <Vec::<i32>>::new();
    for i in 0..vector[0].len() {
        let mut j = 0;
        for row in vector.clone() {
            j += row[i];
        }
        g.push(if j > vector.len() as i32 / 2 { 1 } else { 0 });
        e.push(if j > vector.len() as i32 / 2 { 0 } else { 1 });
    }
    return (g, e)
}