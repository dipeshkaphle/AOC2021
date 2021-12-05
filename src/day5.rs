use std::{collections::HashMap, io::BufRead};

use crate::get_buffer;

pub struct Day5 {}

impl Day5 {
    fn inp() -> Vec<Vec<i32>> {
        let reader = get_buffer("day5.txt");
        reader
            .lines()
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .split("->")
                    .map(|x| x.trim().to_owned())
                    .map(|x| {
                        x.split(",")
                            .map(|y| y.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .flatten()
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }

    fn common(take_diagonal: bool) -> usize {
        let lines = Self::inp();
        // println!("{:?}", lines);
        let mut mp: HashMap<i32, HashMap<i32, usize>> = HashMap::new();
        for line in &lines {
            let delta_x = line[2] - line[0];
            let delta_y = line[3] - line[1];
            if !take_diagonal {
                if !(delta_x == 0 || delta_y == 0) {
                    continue;
                }
            }
            // println!("{:?}", line);
            let divisor = (delta_y.abs()).max(delta_x.abs());
            let mut x = line[0];
            let mut y = line[1];
            // println!("{},{},{}", delta_x, delta_y, divisor);
            // println!("-----------------");

            // println!("{},{}", x, y);
            *(mp.entry(x).or_insert(HashMap::new()).entry(y).or_insert(0)) += 1;
            while !(x == line[2] && y == line[3]) {
                x += (delta_x / (divisor));
                y += (delta_y / (divisor));
                // println!("{},{}", x, y);
                *(mp.entry(x).or_insert(HashMap::new()).entry(y).or_insert(0)) += 1;
            }
            // println!("-----------------");
        }
        let mut cnt = 0;
        for (_x, rows) in &mp {
            // println!("{}:::: {:?}", _x, rows);
            for (_y, vals) in rows {
                if vals >= &2 {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    pub fn part_1() -> usize {
        return Self::common(false);
    }
    pub fn part_2() -> usize {
        return Self::common(true);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::Day5;
        println!("{}", Day5::part_1());
        println!("{}", Day5::part_2());
    }
}
