use std::io::BufRead;

use crate::get_buffer;

pub struct Day6 {}
impl Day6 {
    fn input() -> Vec<i32> {
        let mut reader = get_buffer("input/day6.txt");
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        s.trim()
            .split(",")
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }

    pub fn common(end: usize) -> usize {
        let mut counts = [0 as usize; 9];
        for x in Self::input() {
            counts[x as usize] += 1;
        }
        for _ in 0..end {
            let new_cnt = counts[0];
            for x in 0..=7 {
                counts[x] = counts[x + 1];
            }
            counts[6] += new_cnt;
            counts[8] = new_cnt;
        }
        counts.into_iter().sum()
    }

    pub fn part_1() -> usize {
        Self::common(80)
    }
    pub fn part_2() -> usize {
        Self::common(256)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::Day6;
        println!("{}", Day6::part_1());
        println!("{}", Day6::part_2());
    }
}
