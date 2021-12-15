use std::{collections::HashSet, io::BufRead};

use crate::get_buffer;

pub struct Day13 {}
impl Day13 {
    fn read() -> (Vec<Vec<i32>>, Vec<(u8, usize)>) {
        //
        let mut lines = get_buffer("input/day13.txt")
            .lines()
            .map(|x| x.unwrap())
            .into_iter();
        let mut vec = vec![];
        loop {
            let next_ln = lines.next();
            if let Some(x) = next_ln {
                if x.is_empty() {
                    break;
                }
                let y = x
                    .split(",")
                    .into_iter()
                    .map(|z| z.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                vec.push(y);
            } else {
                break;
            }
        }
        let mut folds = vec![];
        loop {
            let next_ln = lines.next();
            if let Some(x) = next_ln {
                let mut y = x
                    .split_whitespace()
                    .into_iter()
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .chars();
                let fst = y.next().unwrap();
                y.next();
                let coord = y.collect::<String>().parse::<usize>().unwrap();
                if fst == 'y' {
                    folds.push((1, coord))
                } else {
                    folds.push((0, coord))
                }
            } else {
                break;
            }
        }

        (vec, folds)
    }

    fn filter_mapper(coords: HashSet<Vec<i32>>, axis: u8, line: usize) -> HashSet<Vec<i32>> {
        coords
            .into_iter()
            .filter_map(|x| {
                if x[axis as usize] == line as i32 {
                    None
                } else if x[axis as usize] < line as i32 {
                    Some(x)
                } else {
                    let dist = x[axis as usize] - line as i32;
                    if dist < 0 {
                        None
                    } else {
                        let mut cp = x.clone();
                        cp[axis as usize] = line as i32 - dist;
                        Some(cp)
                    }
                }
            })
            .collect::<HashSet<Vec<i32>>>()
    }

    pub fn part_1() -> usize {
        let (coords, folds) = Self::read();
        let (axis, line) = folds[0];
        let coords = coords.into_iter().collect::<HashSet<Vec<i32>>>();
        Self::filter_mapper(coords, axis, line).len()
    }
    pub fn part_2() -> usize {
        let (coords, folds) = Self::read();
        let mut coords = coords.into_iter().collect::<HashSet<Vec<i32>>>();
        for (axis, line) in folds {
            coords = Self::filter_mapper(coords, axis, line);
        }
        let mut mx = 0;
        coords.iter().for_each(|vec| {
            mx = mx.max(vec[0]).max(vec[1]);
        });
        mx += 1;
        for i in 0..mx {
            for j in 0..mx {
                if coords.contains(&vec![j, i]) {
                    print!("{}", "#");
                } else {
                    print!("{}", " ");
                }
            }
            println!("");
        }
        coords.len()
    }
}
#[cfg(test)]
mod tests {

    use super::Day13;

    #[test]
    fn test() {
        println!("{}", Day13::part_1());
        println!("{}", Day13::part_2());
    }
}
