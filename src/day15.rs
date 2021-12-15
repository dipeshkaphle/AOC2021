use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

use crate::get_buffer;

pub struct Day15 {}

impl Day15 {
    fn read() -> Vec<Vec<usize>> {
        get_buffer("input/day15.txt")
            .lines()
            .map(|x| {
                x.unwrap()
                    .chars()
                    .map(|y| y.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn get_neighbors(
        i: i32,
        j: i32,
        mat: &Vec<Vec<usize>>,
        m: usize,
        n: usize,
    ) -> Vec<(usize, usize)> {
        let dx = [-1, 1, 0, 0];
        let dy = [0, 0, -1, 1];
        let mut vec = vec![];
        for k in 0..4 {
            let x = i + dx[k];
            let y = j + dy[k];
            if x >= 0 && y >= 0 && x < (m as i32) && y < (n as i32) {
                vec.push((x as usize, y as usize));
            }
        }
        vec
    }

    fn dijkstra(dest: (usize, usize), mat: &Vec<Vec<usize>>, m: usize, n: usize) -> usize {
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, (0, 0))));
        let mut ans = 0;
        let mut visited = HashSet::new();
        while !pq.is_empty() {
            let (cost, (x, y)) = pq.peek().unwrap().clone().0;
            if (x, y) == dest {
                ans = cost;
                break;
            }
            pq.pop();
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            for (x_, y_) in Self::get_neighbors(x as i32, y as i32, &mat, m, n) {
                if !visited.contains(&(x_, y_)) {
                    pq.push(Reverse((cost + mat[x_][y_], (x_, y_))));
                }
            }
        }
        ans
    }
    fn expand() -> Vec<Vec<usize>> {
        let mat = Self::read();
        let m = mat.len();
        let n = mat[0].len();
        let new_m = 5 * m;
        let new_n = 5 * n;
        let mut new_mat = vec![vec![0; new_n]; new_m];
        for i in 0..m {
            for j in 0..n {
                new_mat[i][j] = mat[i][j];
            }
        }
        for i in 0..m {
            for j in n..new_n {
                new_mat[i][j] = new_mat[i][j - n] + 1;
                if new_mat[i][j] == 10 {
                    new_mat[i][j] = 1;
                }
            }
        }
        for i in m..new_m {
            for j in 0..new_n {
                new_mat[i][j] = new_mat[i - m][j] + 1;
                if new_mat[i][j] == 10 {
                    new_mat[i][j] = 1;
                }
            }
        }
        return new_mat;
    }

    pub fn part_1() -> usize {
        let mat = Self::read();
        let m = mat.len();
        let n = mat[0].len();
        let dest = (m - 1, n - 1);
        Self::dijkstra(dest, &mat, m, n)
    }

    pub fn part_2() -> usize {
        let mat = Self::expand();
        let m = mat.len();
        let n = mat[0].len();
        let dest = (m - 1, n - 1);
        Self::dijkstra(dest, &mat, m, n)
    }
}

#[cfg(test)]
mod tests {

    use super::Day15;

    #[test]
    fn test() {
        println!("{}", Day15::part_1());
        println!("{}", Day15::part_2());
    }
}
