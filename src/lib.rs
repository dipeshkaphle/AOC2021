use std::{
    fs::{self, File},
    io::BufReader,
};
pub fn get_buffer(filename: &str) -> BufReader<File> {
    let f = fs::File::open(filename).unwrap();
    let reader = BufReader::new(f);
    reader
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod dsu;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
