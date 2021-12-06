pub struct Day1 {}
use std::io::BufRead;

use crate::get_buffer;

impl Day1 {
    pub fn common(window_size: usize) -> i32 {
        let reader = get_buffer("input/day1.txt");
        let vec = reader
            .lines()
            .into_iter()
            .map(|x| x.unwrap())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let x = vec
            .windows(window_size as usize + 1)
            .into_iter()
            .filter(|y| {
                (&y[0..(window_size)]).iter().sum::<i32>() - (&y[1..]).iter().sum::<i32>() < 0
            })
            .count();
        x as i32
    }
    pub fn part_1() -> i32 {
        Day1::common(1)
    }
    pub fn part_2() -> i32 {
        Day1::common(3)
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day1;
        println!("{}", Day1::part_1());
        println!("{}", Day1::part_2());
    }
}
