use std::io::BufRead;

use crate::get_buffer;
pub struct Day2 {}

impl Day2 {
    pub fn part_1() -> i32 {
        let reader = get_buffer("input/day2.txt");
        let mut x = 0;
        let mut y = 0;
        reader.lines().into_iter().for_each(|a| {
            let c = a.unwrap();
            let split = c
                .split_whitespace()
                .into_iter()
                .map(|b| b.to_owned())
                .collect::<Vec<String>>();
            let delta = split[1].parse::<i32>().unwrap();
            if &split[0] == "forward" {
                x += delta;
            } else if &split[0] == "down" {
                y += delta;
            } else {
                y -= delta;
            }
        });
        x * y
    }

    pub fn part_2() -> i32 {
        let reader = get_buffer("input/day2.txt");
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;
        reader.lines().into_iter().for_each(|a| {
            let c = a.unwrap();
            let split = c
                .split_whitespace()
                .into_iter()
                .map(|b| b.to_owned())
                .collect::<Vec<String>>();
            let delta = split[1].parse::<i32>().unwrap();
            if &split[0] == "forward" {
                x += delta;
                y += aim * delta;
            } else if &split[0] == "down" {
                aim += delta;
            } else {
                aim -= delta;
            }
        });
        x * y
        // unimplemented!()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day2;
        println!("{}", Day2::part_1());
        println!("{}", Day2::part_2());
    }
}
