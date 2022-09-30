use std::{io::{BufReader, Seek, SeekFrom, Read}, fs::File};

use super::boot_record::BootRecord;




#[derive(Debug)]
pub struct Partition<'a> {
    pub boot_records: [super::boot_record::BootRecord; 2],
    pub first_block_group: super::block_group::BlockGroup<'a>,

}

impl <'a> Partition<'a> {
    pub fn read(from: &mut BufReader<File>) -> Self {
        // set buffer to read from start no matter what
        from.seek(SeekFrom::Start(0)).expect("BAD SEEK TO 0");
        // read boot record byte arrays
        let mut boot_records = [[0 as u8; 512] as BootRecord; 2];
        from.read_exact(&mut boot_records[0]).expect("BAD READ ON IMG WHILE LOADING BOOT RECORDS");
        from.read_exact(&mut boot_records[1]).expect("BAD READ ON IMG WHILE LOADING BOOT RECORDS");
        
        let first_block_group = todo!() as super::block_group::BlockGroup;
        Self {
            boot_records,
            first_block_group
        }
    }
}