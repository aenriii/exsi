use std::{rc::Rc, io::{BufReader, SeekFrom, Seek, Read}, fs::File};
use log::warn;

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

        let mut bgdt_bin: &[u8];
        let mut block_bitmap_bin: &[u8];
        let mut inode_bitmap_bin: &[u8];
        let mut inode_table_bin: &[u8];
        bufReader.take(superblock.block_size()).read(&mut bgdt_bin).expect("BAD READ ON IMG (Loading bgdt ran into an error)");
        bufReader.take(superblock.block_size()).read(&mut block_bitmap_bin).expect("BAD READ ON IMG (Loading block bitmap ran into an error)");
        bufReader.take(superblock.block_size()).read(&mut inode_bitmap_bin).expect("BAD READ ON IMG (Loading inode bitmap ran into an error)");
        
        // inode size * num inodes / block size = num blocks in inode table
        // inodes 


        let t = Self {
            previous: None,
            superblock: Some(superblock),
            bgdt: None,
            block_bitmap: todo!(),
            inode_bitmap: todo!(),
            inode_table: todo!(),
            data_blocks: vec![],
            next: None
        };
        }
        todo!()
    }
    pub fn read_with_prev(bufReader: &mut BufReader<File>, prev: &'a Self) -> Self {


        todo!()
    }

}