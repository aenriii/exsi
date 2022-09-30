

use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct Ext2Image<'lifetime> {
    buffer: BufReader<File>, // file handle to the image file
    partition: super::data::partition::Partition<'lifetime>,
}
impl <'lifetime> Ext2Image<'lifetime> {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect(
            format!("Failed to open file {}", filename).as_str()
        );
        let mut buffer = BufReader::new(file);
        let part = super::data::partition::Partition::read(&mut buffer);
        Self {
            buffer,
            partition: part,
        }
    }
}