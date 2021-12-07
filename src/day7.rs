use std::{cmp::min_by_key, io::BufRead};

use crate::get_buffer;

pub struct Day7 {}
impl Day7 {
    fn read() -> Vec<usize> {
        get_buffer("input/day7.txt")
            .lines()
            .into_iter()
            .map(|x| {
                return x
                    .unwrap()
                    .trim()
                    .split(",")
                    .into_iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            })
            .flatten()
            .collect::<Vec<usize>>()
    }

    fn common(f: fn(usize) -> usize) -> usize {
        let nums = Self::read();
        (0..(*nums.iter().max().unwrap()))
            .map(|x| {
                nums.iter()
                    .map(|y| f((x as i64 - *y as i64).abs() as usize))
                    .sum::<usize>()
            })
            .min()
            .unwrap()
    }

    fn part_1() -> usize {
        Self::common(|x| x)
    }
    fn part_2() -> usize {
        Self::common(|x| (x * (x + 1)) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::Day7;

    #[test]
    fn test() {
        println!("{}", Day7::part_1());
        println!("{}", Day7::part_2());
        // println!("{")
    }
}
