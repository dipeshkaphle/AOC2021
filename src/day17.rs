use std::io::BufRead;

use crate::get_buffer;

enum Velocity {
    X(i64),
    Y(i64),
}
impl Velocity {
    pub fn change(&mut self) {
        match self {
            Self::X(x) => {
                if *x < 0 {
                    *self = Self::X(*x + 1);
                } else if *x > 0 {
                    *self = Self::X(*x - 1);
                }
            }
            Self::Y(y) => {
                *self = Self::Y(*y - 1);
            }
        }
    }
    pub fn get(&self) -> i64 {
        match self {
            Self::X(x) => *x,
            Self::Y(y) => *y,
        }
    }
}

pub struct Day17 {}

impl Day17 {
    fn read() -> (i64, i64, i64, i64) {
        let lines = get_buffer("input/day17.txt")
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let first_line = lines[0].clone();
        let (_, useful) = first_line.split_once(": ").unwrap();
        let (x, y) = useful.split_once(", ").unwrap();
        let (x_min, x_max) = x.split_once("=").unwrap().1.split_once("..").unwrap();
        let (x_min, x_max) = (x_min.parse::<i64>().unwrap(), x_max.parse::<i64>().unwrap());
        let (y_min, y_max) = y.split_once("=").unwrap().1.split_once("..").unwrap();
        let (y_min, y_max) = (y_min.parse::<i64>().unwrap(), y_max.parse::<i64>().unwrap());
        (x_min, x_max, y_min, y_max)
    }

    fn get_valid_times_from_x(x: i64, x_min: i64, x_max: i64) -> Vec<i64> {
        let mut x_vel = Velocity::X(x);
        let mut x_pos = 0;
        let mut valid = vec![];
        let mut i = 0;
        loop {
            i += 1;
            x_pos += x_vel.get();
            if x_pos >= x_min && x_pos <= x_max {
                valid.push(i);
            }
            if x_pos > x_max {
                break;
            }
            let prev_vel = x_vel.get();
            x_vel.change();
            if x_vel.get() == prev_vel {
                break;
            }
        }
        valid
    }

    fn get_all_possible_x(x_min: i64, x_max: i64) -> Vec<i64> {
        let mut vec = vec![];
        for x in 0..(x_max + 1) {
            let valid_times = Self::get_valid_times_from_x(x, x_min, x_max);
            if !valid_times.is_empty() {
                vec.push(x);
            }
        }
        vec
    }

    fn simulate(x: i64, y: i64, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> (bool, i64) {
        let mut v_x = Velocity::X(x);
        let mut v_y = Velocity::Y(y);
        let mut x = 0;
        let mut y = 0;
        let mut mx_y = 0;
        loop {
            x += v_x.get();
            y += v_y.get();
            mx_y = y.max(mx_y);
            if x >= x_min && y >= y_min && x <= x_max && y <= y_max {
                return (true, mx_y);
            }
            if x > x_max || y < y_min {
                break;
            }
            v_x.change();
            v_y.change();
        }
        (false, 0)
    }

    pub fn part_1() -> i64 {
        let (x_min, x_max, y_min, y_max) = Self::read();
        let all_vel = Self::get_all_possible_x(x_min, x_max);
        let mut mx_y = 0;
        for x in all_vel {
            for y in -1000..=1000 {
                let (_, mx_ht) = Self::simulate(x, y, x_min, x_max, y_min, y_max);
                mx_y = mx_y.max(mx_ht);
            }
        }
        mx_y
    }

    pub fn part_2() -> i64 {
        let (x_min, x_max, y_min, y_max) = Self::read();
        let all_vel = Self::get_all_possible_x(x_min, x_max);
        let mut cnt = 0;
        for x in all_vel {
            // I didnt want to do this but couldnt think of anything else
            for y in -1000..=1000 {
                let (valid, _) = Self::simulate(x, y, x_min, x_max, y_min, y_max);
                if valid {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {

    use super::Day17;

    #[test]
    fn test() {
        println!("{}", Day17::part_1());
        println!("{}", Day17::part_2());
    }
}
