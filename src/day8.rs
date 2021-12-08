use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use crate::get_buffer;

pub struct Day8 {}
impl Day8 {
    fn read() -> Vec<Vec<Vec<String>>> {
        get_buffer("input/day8.txt")
            .lines()
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .split("|")
                    .into_iter()
                    .map(|y| {
                        y.trim()
                            .split_whitespace()
                            .map(|z| z.to_string())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>()
            })
            .collect()
    }

    fn intersect(a: &str, b: &str) -> Vec<u8> {
        let a_ = a.as_bytes().iter().copied().collect::<HashSet<u8>>();
        let b_ = b.as_bytes().iter().copied().collect::<HashSet<u8>>();
        a_.intersection(&b_).copied().collect::<Vec<u8>>()
    }

    fn union(a: &str, b: &str) -> Vec<u8> {
        let a_ = a.as_bytes().iter().copied().collect::<HashSet<u8>>();
        let b_ = b.as_bytes().iter().copied().collect::<HashSet<u8>>();
        a_.union(&b_).copied().collect::<Vec<u8>>()
    }

    pub fn part_1() -> usize {
        let uniq_ones = vec![2, 3, 4, 7];
        let mut cnt = 0;
        Self::read().into_iter().for_each(|x| {
            x[1].iter().for_each(|y| {
                if uniq_ones.iter().find(|z| **z == y.len()).is_some() {
                    cnt += 1;
                }
            });
        });
        cnt
    }

    pub fn part_2() -> usize {
        let _mapper = vec![
            ("abcdefg", 8),
            ("bcdf", 4),
            ("acf", 7),
            ("cf", 1),
            ("abcefg", 0),
            ("abcdfg", 9),
            ("abdefg", 6),
            ("abdfg", 5),
            ("acdeg", 2),
            ("acdfg", 3),
        ]
        .into_iter()
        .collect::<HashMap<&str, usize>>();

        let inp = Self::read();
        let mut final_ans = 0;
        for line in inp {
            let mut len_to_strings = HashMap::new();
            for s in line[0].iter() {
                len_to_strings
                    .entry(s.len())
                    .or_insert(vec![])
                    .push(s.to_owned());
            }
            let mut ans = 0;
            for s in line[1].iter() {
                let f: usize = match s.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    6 => {
                        if Self::intersect(&len_to_strings.get(&2).unwrap()[0], &s).len() != 2 {
                            6
                        } else {
                            let x = &len_to_strings.get(&3).unwrap()[0];
                            let y = &len_to_strings.get(&4).unwrap()[0];
                            let x_union_y = Self::union(x, y)
                                .into_iter()
                                .map(|x| x as char)
                                .collect::<String>();
                            if Self::intersect(&x_union_y, &s).len() == 5 {
                                9
                            } else {
                                0
                            }
                        }
                    }
                    5 => {
                        if Self::intersect(&len_to_strings.get(&2).unwrap()[0], &s).len() == 2 {
                            3
                        } else {
                            if Self::intersect(&len_to_strings.get(&4).unwrap()[0], &s).len() == 2 {
                                2
                            } else {
                                5
                            }
                        }
                    }
                    _ => unimplemented!(),
                };
                ans *= 10;
                ans += f;
            }
            final_ans += ans;
        }
        final_ans
    }
}

#[cfg(test)]
mod tests {
    use super::Day8;

    #[test]
    fn test() {
        println!("{}", Day8::part_1());
        println!("{}", Day8::part_2());
    }
}
