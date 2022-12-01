use lib::read_lines;

fn main() {
    let lines = read_lines();
    let mut gamma = Vec::<i32>::new();
    let mut epsilon = Vec::<i32>::new();
    for line in lines {
        let mut column = Vec::<char>::new();
        for char in line.chars() {
            column.push(char);
        }
        let mut one = 0;
        for char in &column {
            if char == &'1' {
                one += 1
            }
        }
        if one > (column.len()/2) {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1)
        }
    }

    println!("{}", )
    let mut gamma_int = 0;
    let mut epsilon_int = 0;
    for i in gamma.len()..0 {
        gamma_int += (2_i32.pow(i.try_into().unwrap()))*gamma[i];
        epsilon_int += (2_i32.pow(i.try_into().unwrap()))*epsilon[i];
    }
    println!("{}", gamma_int*epsilon_int);
}