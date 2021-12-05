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
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
