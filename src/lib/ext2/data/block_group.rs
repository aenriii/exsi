use std::{rc::Rc, io::{BufReader, SeekFrom, Seek, Read}, fs::File};
use log::warn;

use crate::lib::traits::ReadFrom;

use super::{
    bgdt::BlockGroupDescriptor, 
    superblock::Superblock,
    block_bitmap::BlockBitmap,
    inode_bitmap::InodeBitmap,
    inode_table::InodeTable, data_block::DataBlock,
};

#[derive(Debug)]
pub struct BlockGroup {
    // pub previous: Option<&'a BlockGroup<'a>>, // unnecessary with new design
    //pub superblock: Option<super::superblock::Superblock>, // moving to Partition
    //pub bgdt: Option<super::bgdt::BlockGroupDescriptorTable>, // moving to Partition
    pub block_bitmap: super::block_bitmap::BlockBitmap,
    pub inode_bitmap: super::inode_bitmap::InodeBitmap,
    pub inode_table: super::inode_table::InodeTable,
    pub data_blocks: Vec<super::data_block::DataBlock>,
    // pub next: Option<&'a BlockGroup<'a>> // unnecessary with new design
}

impl BlockGroup {
    pub fn read(superblock: &Superblock, descriptor: BlockGroupDescriptor, reader: &mut BufReader<File>) -> Self {
        let block_size = superblock.block_size() as u32;
        // seek to block bitmap
        reader.seek(SeekFrom::Start(descriptor.block_bitmap as u64 * block_size as u64)).unwrap();
        let block_bitmap = BlockBitmap::read(reader, block_size, superblock);
        // seek to inode bitmap
        reader.seek(SeekFrom::Start(descriptor.inode_bitmap as u64 * block_size as u64)).unwrap();
        let inode_bitmap = InodeBitmap::read(reader, block_size, superblock);
        // seek to inode table
        reader.seek(SeekFrom::Start(descriptor.inode_table as u64 * block_size as u64)).unwrap();
        let inode_table = InodeTable::read(reader, block_size, superblock);
        
        let data_blocks: Vec<DataBlock> = Vec::new();

        Self {
            block_bitmap,
            inode_bitmap,
            inode_table,
            data_blocks,
        }
    }

}