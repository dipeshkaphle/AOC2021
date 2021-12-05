use std::{collections::HashSet, io::BufRead};

use crate::get_buffer;

pub struct Day4 {}
impl Day4 {
    pub fn read_grids() -> (Vec<usize>, Vec<Vec<Vec<(usize, bool)>>>) {
        let reader = get_buffer("day4.txt");
        let mut iters = reader.lines().into_iter();
        let first_line = iters
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut grids = vec![];
        loop {
            let ln = iters.next();
            if ln.is_none() {
                break;
            }
            let mut this_grid = vec![];
            for _ in 0..5 {
                let ln = iters
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| (x.parse::<usize>().unwrap(), false))
                    .collect::<Vec<(usize, bool)>>();
                this_grid.push(ln);
            }
            grids.push(this_grid);
        }
        (first_line, grids)
    }

    fn compute_val(grid: &Vec<Vec<(usize, bool)>>) -> usize {
        let mut sm = 0;
        for i in 0..5 {
            for j in 0..5 {
                if grid[i][j].1 == false {
                    sm += grid[i][j].0;
                }
            }
        }
        sm
    }

    fn is_done(grid: &Vec<Vec<(usize, bool)>>) -> bool {
        for i in 0..5 {
            let mut all_done = true;
            for j in 0..5 {
                all_done = all_done & grid[i][j].1;
            }
            if all_done {
                return true;
            }
        }
        for j in 0..5 {
            let mut all_done = true;
            for i in 0..5 {
                all_done = all_done & grid[i][j].1;
            }
            if all_done {
                return true;
            }
        }
        return false;
    }

    pub fn part_1() -> usize {
        let (numbers, mut grids) = Day4::read_grids();
        for x in &numbers {
            let mut done = vec![];
            for k in 0..(grids.len()) {
                let g = &mut grids[k];
                for i in 0..5 {
                    for j in 0..5 {
                        if g[i][j].0 == *x {
                            g[i][j].1 = true;
                        }
                    }
                }
                if Self::is_done(g) {
                    done.push(k);
                }
            }
            // println!("{}", x);
            // println!("{:?}", grids);
            if !done.is_empty() {
                let a = done
                    .iter()
                    .map(|y| Self::compute_val(&grids[*y]))
                    .max()
                    .unwrap();
                return a * (*x);
            }
        }
        0
    }
    pub fn part_2() -> usize {
        let (numbers, mut grids) = Day4::read_grids();
        let mut last_done = vec![];
        let mut already_done = HashSet::new();
        let mut last = 0;
        for x in &numbers {
            let mut done = vec![];
            for k in 0..(grids.len()) {
                let g = &mut grids[k];
                for i in 0..5 {
                    for j in 0..5 {
                        if g[i][j].0 == *x {
                            g[i][j].1 = true;
                        }
                    }
                }
                if !already_done.contains(&k) && Self::is_done(g) {
                    done.push(k);
                }
            }
            if !(&done).is_empty() {
                last_done = vec![];
                done.iter().for_each(|a| {
                    already_done.insert(*a);
                    last_done.push(grids[*a].clone());
                });
                last = *x;
            }
        }
        let a = last_done
            .iter()
            .map(|y| Self::compute_val(y))
            .max()
            .unwrap();
        // println!("{:?}, {}, {}", last_done, last, a);
        return a * (last);
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::Day4;
        println!("{}", Day4::part_1());
        println!("{}", Day4::part_2());
        // println!("{}", Day3::part_2());
    }
}
