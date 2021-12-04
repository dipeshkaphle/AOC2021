pub struct Day1 {}
use std::{
    fs,
    io::{BufRead, BufReader},
};

use crate::get_buffer;

impl Day1 {
    pub fn part_1() -> i32 {
        let reader = get_buffer("day1.txt");
        let vec = reader
            .lines()
            .into_iter()
            .map(|x| x.unwrap())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let ans = &vec[..(vec.len() - 1)]
            .iter()
            .zip(&vec[1..])
            .map(|(x, y)| y - x)
            .filter(|x| x > &0)
            .map(|_x| 1)
            .sum::<i32>();
        return ans.to_owned();
    }
    pub fn part_2() -> i32 {
        let reader = get_buffer("day1.txt");
        let vec = reader
            .lines()
            .into_iter()
            .map(|x| x.unwrap())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let grouped = &vec[..]
            .iter()
            .zip(&vec[1..])
            .zip(&vec[2..])
            .map(|((x, y), z)| x + y + z)
            .collect::<Vec<i32>>();
        let ans = &grouped[..(grouped.len() - 1)]
            .iter()
            .zip(&grouped[1..])
            .map(|(x, y)| y - x)
            .filter(|x| x > &0)
            .map(|_x| 1)
            .sum::<i32>();
        return ans.to_owned();
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
