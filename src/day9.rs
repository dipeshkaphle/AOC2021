use std::{collections::HashSet, io::BufRead};

use crate::{dsu::DSU, get_buffer};

pub struct Day9 {}
impl Day9 {
    fn read() -> Vec<Vec<u8>> {
        get_buffer("input/day9.txt")
            .lines()
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .as_str()
                    .as_bytes()
                    .into_iter()
                    .map(|y| *y as u8 - '0' as u8)
                    .collect()
            })
            .collect()
    }
    //

    fn access(arr: &Vec<Vec<u8>>, i: i32, j: i32) -> Option<u8> {
        if i >= 0 && j >= 0 && (i as usize) < arr.len() && (j as usize) < arr[0].len() {
            return Some(arr[i as usize][j as usize]);
        }
        None
    }
    pub fn get_minimum_in_the_hood(arr: &Vec<Vec<u8>>, i: i32, j: i32) -> (u8, (usize, usize)) {
        let mut values = vec![];
        let mut indices = vec![];
        for (x, y) in [(i, j), (i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            let elem = Self::access(arr, x, y);
            if elem.is_some() {
                values.push(elem.unwrap());
                indices.push((x as usize, y as usize));
            }
        }
        let min_ = values.iter().min().unwrap();
        let min_at = values.iter().position(|x| x == min_).unwrap();
        return (*min_, indices[min_at]);
    }

    fn part_1_helper(f: fn([Option<u8>; 4], u8) -> bool) -> usize {
        let inputs = Self::read();
        let m = inputs.len();
        let n = inputs[0].len();
        let mut points = vec![];
        for i in 0..(m as i32) {
            for j in 0..(n as i32) {
                if f(
                    [
                        Self::access(&inputs, i - 1, j),
                        Self::access(&inputs, i + 1, j),
                        Self::access(&inputs, i, j + 1),
                        Self::access(&inputs, i, j - 1),
                    ],
                    inputs[i as usize][j as usize],
                ) {
                    points.push(inputs[i as usize][j as usize]);
                }
            }
        }
        points.into_iter().map(|x| (x + 1) as usize).sum::<usize>()
    }

    pub fn part_1() -> usize {
        Self::part_1_helper(|vec, x| {
            vec.into_iter().all(|y| {
                if let Some(z) = y {
                    return z > x;
                }
                return true;
            })
        })
    }

    fn initiate_dsu(m: usize, n: usize) -> DSU<(usize, usize)> {
        let mut dsu = DSU::new();
        for i in 0..m {
            for j in 0..n {
                dsu.new_entry(&(i.clone(), j.clone()));
            }
        }
        dsu
    }

    pub fn part_2() -> usize {
        //
        let inputs = Self::read();
        let m = inputs.len();
        let n = inputs[0].len();
        let mut dsu = Self::initiate_dsu(m, n);
        for i in 0..m {
            for j in 0..n {
                if inputs[i][j] == 9 {
                    continue;
                }
                let (_, index) = Self::get_minimum_in_the_hood(&inputs, i as i32, j as i32);
                dsu.union(&(i, j), &index);
            }
        }
        let mut all_leaders = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                all_leaders.insert(dsu.get_par(&(i, j)));
            }
        }

        let mut all_sizes = vec![];
        for (x, y) in all_leaders.iter() {
            all_sizes.push(dsu.size[&(x.clone(), y.clone())]);
        }
        all_sizes.sort();
        all_sizes.iter().rev().take(3).product::<usize>()
    }
}

#[cfg(test)]
mod tests {

    use super::Day9;

    #[test]
    fn test() {
        println!("{}", Day9::part_1());
        println!("{}", Day9::part_2());
    }
}
