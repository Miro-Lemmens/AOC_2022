impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut reversed: u32 = 0;
        while x != 0 {
            reversed = reversed * 10  + x % 10;
            x /= 10;
        }
        let mut numstr: &str = reversed.to_string();
        let intval = isize::from_str_radix(numstr, 2).unwrap();
        println!("{}", intval);
    }
}
