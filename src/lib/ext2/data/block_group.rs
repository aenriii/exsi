use std::{rc::Rc, io::{BufReader, SeekFrom, Seek, Read}, fs::File};
use log::warn;

use crate::lib::{ext2::data::{bgdt::BlockGroupDescriptorTable, block_bitmap::BlockBitmap, inode_bitmap::InodeBitmap, inode_table::InodeTable, data_block::DataBlock}, traits::FromBin};

#[derive(Debug)]
pub struct BlockGroup<'a> {
    pub previous: Option<&'a BlockGroup<'a>>, 
    pub superblock: Option<super::superblock::Superblock>,
    pub bgdt: Option<super::bgdt::BlockGroupDescriptorTable>,
    pub block_bitmap: super::block_bitmap::BlockBitmap,
    pub inode_bitmap: super::inode_bitmap::InodeBitmap,
    pub inode_table: super::inode_table::InodeTable,
    pub data_blocks: Vec<super::data_block::DataBlock>,
    pub next: Option<&'a BlockGroup<'a>>
}

impl <'a> BlockGroup<'a> {
    pub fn read(bufReader: &mut BufReader<File>) -> Self {

        warn!("BlockGroup::read() (as opposed to BlockGroup::read_with_prev()) assumes this is the first block group and will seek directly to 1024 bytes into the image. If you see this more than once something is wrong.");     
        bufReader.seek(SeekFrom::Start(1024)).expect("BAD SEEK TO 1024");
        
        // superblock
        let mut superblock_bin = [0 as u8; 1024];
        bufReader.read_exact(&mut superblock_bin).expect("BAD READ ON IMG (Loading superblock failed on initial blockgroup)");
        let superblock = super::superblock::Superblock::read_from_bin(superblock_bin);
        // superblock gives us block size, so we can get everything else now
        println!("{:?}", superblock);
        panic!("ending here, for testing");

        let bgdt_bin: &[u8] = &[];
        let block_bitmap_bin: &[u8] = &[];
        let inode_bitmap_bin: &[u8] = &[];
        let mut inode_table_bin: &[u8] = &[];
        // seek to bgdt starting place
        if superblock.block_size() == 1024 {
            bufReader.seek(SeekFrom::Start(1024)).expect("BAD SEEK TO THIRD BLOCK");
        } else {
            bufReader.seek(SeekFrom::Start(superblock.block_size() * 2)).expect("BAD SEEK TO SECOND BLOCK");
        }
        bufReader.take(superblock.block_size()).read(&mut bgdt_bin.to_owned()).expect("BAD READ ON IMG (Loading bgdt ran into an error)");
        bufReader.take(superblock.block_size()).read(&mut block_bitmap_bin.to_owned()).expect("BAD READ ON IMG (Loading block bitmap ran into an error)");
        bufReader.take(superblock.block_size()).read(&mut inode_bitmap_bin.to_owned()).expect("BAD READ ON IMG (Loading inode bitmap ran into an error)");
        
        // inode size * num inodes / block size = num blocks in inodenum_blocks_in_inode_tablebufReader.take(superblock.block_size() * inode_table_blockcount).read(&mut inode_table_bin).expect("BAD READ ON IMG (Loading inode table ran into an error)");
        let inode_table_blockcount = (superblock.inode_size as u32 * superblock.inodes_per_group) as u64 / superblock.block_size();
        bufReader.take(superblock.block_size() * inode_table_blockcount).read(&mut inode_table_bin.to_owned()).expect("BAD READ ON IMG (Loading inode table ran into an error)");

        
        // read in data blocks
        let mut data_blocks: Vec<DataBlock> = Vec::new();
        for i in 1..superblock.blocks_count {
            todo!()
        }

        Self {
            previous: None,
            superblock: Some(superblock),
            bgdt: Some(BlockGroupDescriptorTable::read_from_bin(bgdt_bin)),
            block_bitmap: BlockBitmap::read_from_bin(block_bitmap_bin),
            inode_bitmap: InodeBitmap::read_from_bin(inode_bitmap_bin),
            inode_table: InodeTable::read_from_bin(inode_table_bin),
            data_blocks: data_blocks,
            next: None
        };
        todo!()
    }
    pub fn read_with_prev(bufReader: &mut BufReader<File>, prev: &'a Self) -> Self {


        todo!();
    }

}