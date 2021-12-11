use std::{collections::VecDeque, io::BufRead};

use crate::get_buffer;

pub struct Day11 {}
impl Day11 {
    pub fn read() -> Vec<Vec<u8>> {
        get_buffer("input/day11.txt")
            .lines()
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .as_bytes()
                    .into_iter()
                    .map(|y| y - '0' as u8)
                    .collect()
            })
            .collect()
    }

    fn get_neighbors(i: i32, j: i32, m: usize, n: usize) -> Vec<(usize, usize)> {
        let dx = [-1, 0, 1];
        let dy = [-1, 0, 1];
        let mut vec = vec![];
        for x in dx {
            for y in dy {
                if x == 0 && y == 0 {
                    continue;
                }
                let new_i = i + x;
                let new_j = j + y;
                if new_i >= 0 && new_j >= 0 && new_i < m as i32 && new_j < n as i32 {
                    vec.push((new_i as usize, new_j as usize));
                }
            }
        }
        vec
    }

    fn bfs(
        inp: &mut Vec<Vec<u8>>,
        mut dq: VecDeque<(usize, usize)>,
        cnt: &mut usize,
        m: usize,
        n: usize,
    ) {
        while !dq.is_empty() {
            let (i, j) = dq.front().unwrap().clone();
            dq.pop_front();
            for (x, y) in Self::get_neighbors(i as i32, j as i32, m, n) {
                if inp[x][y] == 0 {
                    continue;
                } else if inp[x][y] == 9 {
                    inp[x][y] = 0;
                    *cnt += 1;
                    dq.push_back((x, y));
                } else {
                    inp[x][y] += 1;
                }
            }
        }
    }

    fn simulate(inp: &mut Vec<Vec<u8>>, m: usize, n: usize) -> usize {
        let mut cnt = 0;
        let mut dq = VecDeque::new();
        inp.iter_mut().enumerate().for_each(|(i, x)| {
            x.iter_mut().enumerate().for_each(|(j, y)| {
                if y == &9 {
                    *y = 0;
                    cnt += 1;
                    dq.push_back((i, j));
                } else {
                    *y += 1;
                }
            });
        });
        Self::bfs(inp, dq, &mut cnt, m, n);
        cnt
    }

    pub fn part_1() -> usize {
        let mut inp = Self::read();
        let mut cnt = 0;
        let m = inp.len();
        let n = inp[0].len();
        for _ in 0..100 {
            cnt += Self::simulate(&mut inp, m, n);
        }
        cnt
    }
    pub fn part_2() -> usize {
        let mut inp = Self::read();
        let mut cnt = 0;
        let m = inp.len();
        let n = inp[0].len();
        while Self::simulate(&mut inp, m, n) != m * n {
            cnt += 1;
        }
        cnt + 1
    }
}

#[cfg(test)]
mod tests {

    use super::Day11;

    #[test]
    fn test() {
        println!("{}", Day11::part_1());
        println!("{}", Day11::part_2());
    }
}
