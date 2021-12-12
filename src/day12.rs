use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use crate::get_buffer;

pub struct Day12 {}
impl Day12 {
    fn is_upper(s: &str) -> bool {
        s.chars().all(|x| x.is_uppercase())
    }
    fn read() -> HashMap<String, Vec<String>> {
        let mut g = HashMap::new();
        for line in get_buffer("input/day12.txt").lines().into_iter().map(|x| {
            x.unwrap()
                .split("-")
                .map(|y| y.to_owned())
                .collect::<Vec<String>>()
        }) {
            g.entry(line[0].clone())
                .or_insert(vec![])
                .push(line[1].clone());
            g.entry(line[1].clone())
                .or_insert(vec![])
                .push(line[0].clone());
        }
        g
    }
    fn dfs(
        g: &HashMap<String, Vec<String>>,
        node: &str,
        vis: &mut HashSet<String>,
        cnt: &mut usize,
    ) {
        //
        if node == "end" {
            *cnt += 1;
            return;
        }
        vis.insert(node.to_owned());
        for x in &g[node] {
            if vis.contains(x) && !Self::is_upper(x) {
                continue;
            }
            Self::dfs(g, x, vis, cnt);
        }
        vis.remove(node);
    }
    fn dfs_part2(
        g: &HashMap<String, Vec<String>>,
        node: &str,
        vis: &mut HashSet<String>,
        cnt: &mut usize,
        repeated: &str,
    ) {
        if node == "end" {
            *cnt += 1;
            return;
        }
        vis.insert(node.to_owned());
        for x in &g[node] {
            if vis.contains(x) && !Self::is_upper(x) {
                if x != "start" && x != "end" && repeated == "" {
                    Self::dfs_part2(g, x, vis, cnt, x);
                }
                continue;
            }
            Self::dfs_part2(g, x, vis, cnt, repeated);
        }
        if node != repeated {
            vis.remove(node);
        }
    }

    pub fn part_1() -> usize {
        let mut cnt = 0;
        Self::dfs(&Self::read(), "start", &mut HashSet::new(), &mut cnt);
        return cnt;
    }
    pub fn part_2() -> usize {
        let mut cnt = 0;
        Self::dfs_part2(&Self::read(), "start", &mut HashSet::new(), &mut cnt, "");
        // Self::dfs(
        // &Self::read(),
        // "start",
        // &mut HashMap::new(),
        // &|vis: &mut HashMap<String, usize>, x| {
        // return x == "start" || (vis.contains_key(x) && !Self::is_upper(x) && vis[x] == 2);
        // // unimplemented!()
        // },
        // &mut cnt,
        // );
        return cnt;
    }
}

#[cfg(test)]
mod tests {

    use super::Day12;

    #[test]
    fn test() {
        println!("{}", Day12::part_1());
        println!("{}", Day12::part_2());
    }
}
