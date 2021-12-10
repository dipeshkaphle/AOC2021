use std::{collections::HashMap, io::BufRead};

use crate::get_buffer;

enum Status {
    Incomplete(Vec<u8>),
    Invalid(usize),
    Valid,
}

pub struct Day10 {}
impl Day10 {
    fn read() -> Vec<Vec<u8>> {
        get_buffer("input/day10.txt")
            .lines()
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .as_str()
                    .as_bytes()
                    .into_iter()
                    .map(|y| *y)
                    .collect()
            })
            .collect()
    }
    fn process(line: &Vec<u8>) -> Status {
        let mp = vec![
            ('<' as u8, '>' as u8),
            ('(' as u8, ')' as u8),
            ('{' as u8, '}' as u8),
            ('[' as u8, ']' as u8),
        ]
        .into_iter()
        .collect::<HashMap<u8, u8>>();
        let cost = vec![
            (')' as u8, 3),
            (']' as u8, 57),
            ('}' as u8, 1197),
            ('>' as u8, 25137),
        ]
        .into_iter()
        .collect::<HashMap<u8, usize>>();
        let mut st = vec![];
        for x in line {
            if mp.contains_key(x) {
                st.push(*x);
            } else {
                if !st.is_empty() && mp[st.last().unwrap()] == *x {
                    st.pop();
                } else {
                    return Status::Invalid(cost[x]);
                }
            }
        }
        if st.is_empty() {
            Status::Valid
        } else {
            Status::Incomplete(st)
        }
    }

    pub fn complete_braces(st: Vec<u8>) -> usize {
        let mp = vec![
            ('<' as u8, '>' as u8),
            ('(' as u8, ')' as u8),
            ('{' as u8, '}' as u8),
            ('[' as u8, ']' as u8),
        ]
        .into_iter()
        .collect::<HashMap<u8, u8>>();
        let cost = vec![
            (')' as u8, 1),
            (']' as u8, 2),
            ('}' as u8, 3),
            ('>' as u8, 4),
        ]
        .into_iter()
        .collect::<HashMap<u8, usize>>();
        let acc = st.into_iter().rev().fold(vec![], |mut acc, elem| {
            acc.push(cost[&mp[&elem]]);
            acc
        });
        acc.into_iter().fold(0, |acc, elem| acc * 5 + elem)
        // unimplemented!()
    }
    pub fn part_1() -> usize {
        let mut sm = 0;
        for line in Self::read() {
            match Self::process(&line) {
                Status::Invalid(x) => sm += x,
                _ => {}
            }
        }
        sm
    }
    pub fn part_2() -> usize {
        let mut all = vec![];
        for line in Self::read() {
            match Self::process(&line) {
                Status::Incomplete(x) => all.push(Self::complete_braces(x)),
                _ => {}
            }
        }
        all.sort();
        all[all.len() / 2]
    }
}

#[cfg(test)]
mod tests {

    use super::Day10;

    #[test]
    fn test() {
        println!("{}", Day10::part_1());
        println!("{}", Day10::part_2());
    }
}
