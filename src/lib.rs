pub mod header;
pub mod reader;
pub mod writer;
pub mod utils;
pub mod splitter;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    #[test]
    fn it_works() {
        let _ =reader::Reader::from_file(File::open("./test.wav").unwrap());
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
