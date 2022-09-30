
pub type BlockGroupDescriptorTable = Vec<BlockGroupDescriptor>;
#[derive(Debug)]

pub struct BlockGroupDescriptor {
    pub block_bitmap: u32,
    pub inode_bitmap: u32,
    pub inode_table: u32,
    pub free_blocks_count: u16,
    pub free_inodes_count: u16,
    pub used_dirs_count: u16,
    pub pad: u16,
    pub reserved: [u16; 6] // UNNEEDED, should usually be 0x0
}

