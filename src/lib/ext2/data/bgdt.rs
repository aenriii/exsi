
pub type BlockGroupDescriptorTable = Vec<BlockGroupDescriptor>;
#[derive(Debug, Clone, Copy)]

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

impl crate::lib::traits::FromBin for BlockGroupDescriptorTable {
    fn read_from_bin(bin: &[u8]) -> Self {
        let mut bgdt = vec![];
        for i in 0..bin.len() / 32 {
            let mut bgd_bin = [0 as u8; 32];
            bgd_bin.copy_from_slice(&bin[i * 32..(i + 1) * 32]);
            bgdt.push(BlockGroupDescriptor::read_from_bin(&bgd_bin));
        }
        bgdt
    }
}
impl crate::lib::traits::FromBin for BlockGroupDescriptor {
    fn read_from_bin(bin: &[u8]) -> Self {
        Self {
            block_bitmap: u32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]),
            inode_bitmap: u32::from_le_bytes([bin[4], bin[5], bin[6], bin[7]]),
            inode_table: u32::from_le_bytes([bin[8], bin[9], bin[10], bin[11]]),
            free_blocks_count: u16::from_le_bytes([bin[12], bin[13]]),
            free_inodes_count: u16::from_le_bytes([bin[14], bin[15]]),
            used_dirs_count: u16::from_le_bytes([bin[16], bin[17]]),
            pad: u16::from_le_bytes([bin[18], bin[19]]),
            reserved: [
                u16::from_le_bytes([bin[20], bin[21]]),
                u16::from_le_bytes([bin[22], bin[23]]),
                u16::from_le_bytes([bin[24], bin[25]]),
                u16::from_le_bytes([bin[26], bin[27]]),
                u16::from_le_bytes([bin[28], bin[29]]),
                u16::from_le_bytes([bin[30], bin[31]])
            ]
            
        }
    }
}