use std::{io::{BufReader, Seek, SeekFrom, Read}, fs::File};

use crate::lib::traits::FromBin;

use super::{boot_record::BootRecord, block_group::BlockGroup};




#[derive(Debug)]
pub struct Partition {
    pub boot_records: [super::boot_record::BootRecord; 2],
    pub superblock: super::superblock::Superblock,
    pub bgdt: super::bgdt::BlockGroupDescriptorTable,
    pub block_groups: Vec<super::block_group::BlockGroup>,

}

impl  Partition {
    pub fn read(from: &mut BufReader<File>) -> Self {
        // set buffer to read from start no matter what
        from.seek(SeekFrom::Start(0)).expect("BAD SEEK TO 0");
        // read boot record byte arrays
        let mut boot_records = [[0 as u8; 512] as BootRecord; 2];
        from.read_exact(&mut boot_records[0]).expect("BAD READ ON IMG WHILE LOADING BOOT RECORDS");
        from.read_exact(&mut boot_records[1]).expect("BAD READ ON IMG WHILE LOADING BOOT RECORDS");
        

        // read superblock from bin
        let mut sb_bin = [0 as u8; 1024];
        from.read_exact(&mut sb_bin).expect("BAD READ ON IMG WHILE LOADING SUPERBLOCK");
        let superblock = super::superblock::Superblock::read_from_bin(sb_bin);
        // read block group descriptor table via fromraw
        let mut bin = vec![0 as u8; superblock.block_size() as usize];
        from.read_exact(&mut bin).expect("BAD READ ON IMG WHILE LOADING BGDT");
        let bgdt = super::bgdt::BlockGroupDescriptorTable::read_from_bin(&bin);
        // read block groups
        let mut block_groups: Vec<BlockGroup> = Vec::new();
        for x in 0..(superblock.blocks_count / superblock.blocks_per_group) {
            let bg = BlockGroup::read(&superblock, bgdt[x as usize], from);
            block_groups.push(bg);
        }
        Self {
            boot_records,
            superblock,
            bgdt,
            block_groups,
        }
    }
}