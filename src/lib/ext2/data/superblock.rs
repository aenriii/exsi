use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Read};

#[derive(Debug)]
pub struct Superblock {
    pub inodes_count: u32, // verify: <= inodes_per_group * number of block groups
    pub blocks_count: u32,
    pub r_blocks_count: u32, // blocks restricted to superusers
    pub free_blocks_count: u32,
    pub free_inodes_count: u32,
    pub first_data_block: u32, // id of first data block
    pub log_block_size: u32, // verify: >= 0; block_size = 1024 << log_block_size
    pub log_frag_size: i32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
    pub inodes_per_group: u32,
    pub mtime: u32, //last mount
    pub wtime: u32, //last write
    pub mnt_count: u16, //times mounted since last fsck
    pub max_mnt_count: u16, //max times mounted since last fsck
    pub magic: u16, // verify: EXT2_SUPER_MAGIC == 0xEF53
    pub state: u16, // verify: == 1 || 2, VALID | ERROR
    pub errors: u16,
    pub minor_rev_level: u16,
    pub lastcheck: u32, // POSIX unix time of last fsck
    pub checkinterval: u32, // POSIX max unix interval between fscks
    pub creator_os: u32,
    pub rev_level: u32,
    pub def_resuid: u16,
    pub def_resgid: u16,
    pub first_ino: u32,
    pub inode_size: u16,
    pub block_group_nr: u16,
    pub feature_compat: u32,
    pub feature_incompat: u32,
    pub feature_ro_compat: u32,
    pub uuid: u128,
    pub volume_name: [u8; 16],
    pub last_mounted: [u8; 64],
    pub algo_bitmap: u32,
    pub prealloc_blocks: i8,
    pub prealloc_dir_blocks: i8,
    pub journal_uuid: u16,
    pub journal_inum: u32,
    pub journal_dev: u32,
    pub last_orphan: u32,
    pub hash_seed: [u32; 4],
    pub def_hash_version: u8,
    pub default_mount_options: u32,
    pub first_meta_bg: u32
}
impl Superblock {
    pub fn block_size(&self) -> u64 {
        1024 << self.log_block_size
    }
    pub fn frag_size(&self) -> u64 {
        if (self.log_frag_size < 0) {
            1024 << self.log_frag_size
        } else {
            1024 >> -self.log_frag_size
        }
    }

    pub fn read_from_bin(bin: [u8; 1024]) -> Self {
        Self {
            inodes_count: u32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]),
            blocks_count: u32::from_le_bytes([bin[4], bin[5], bin[6], bin[7]]),
            r_blocks_count: u32::from_le_bytes([bin[8], bin[9], bin[10], bin[11]]),
            free_blocks_count: u32::from_le_bytes([bin[12], bin[13], bin[14], bin[15]]),
            free_inodes_count: u32::from_le_bytes([bin[16], bin[17], bin[18], bin[19]]),
            first_data_block: u32::from_le_bytes([bin[20], bin[21], bin[22], bin[23]]),
            log_block_size: u32::from_le_bytes([bin[24], bin[25], bin[26], bin[27]]),
            log_frag_size: i32::from_le_bytes([bin[28], bin[29], bin[30], bin[31]]),
            blocks_per_group: u32::from_le_bytes([bin[32], bin[33], bin[34], bin[35]]),
            frags_per_group: u32::from_le_bytes([bin[36], bin[37], bin[38], bin[39]]),
            inodes_per_group: u32::from_le_bytes([bin[40], bin[41], bin[42], bin[43]]),
            mtime: u32::from_le_bytes([bin[44], bin[45], bin[46], bin[47]]),
            wtime: u32::from_le_bytes([bin[48], bin[49], bin[50], bin[51]]),
            mnt_count: u16::from_le_bytes([bin[52], bin[53]]),
            max_mnt_count: u16::from_le_bytes([bin[54], bin[55]]),
            magic: u16::from_le_bytes([bin[56], bin[57]]),
            state: u16::from_le_bytes([bin[58], bin[59]]),
            errors: u16::from_le_bytes([bin[60], bin[61]]),
            minor_rev_level: u16::from_le_bytes([bin[62], bin[63]]),
            lastcheck: u32::from_le_bytes([bin[64], bin[65], bin[66], bin[67]]),
            checkinterval: u32::from_le_bytes([bin[68], bin[69], bin[70], bin[71]]),
            creator_os: u32::from_le_bytes([bin[72], bin[73], bin[74], bin[75]]),
            rev_level: u32::from_le_bytes([bin[76], bin[77], bin[78], bin[79]]),
            def_resuid: u16::from_le_bytes([bin[80], bin[81]]),
            def_resgid: u16::from_le_bytes([bin[82], bin[83]]),
            first_ino: u32::from_le_bytes([bin[84], bin[85], bin[86], bin[87]]),
            inode_size: u16::from_le_bytes([bin[88], bin[89]]),
            block_group_nr: u16::from_le_bytes([bin[90], bin[91]]),
            feature_compat: u32::from_le_bytes([bin[92], bin[93], bin[94], bin[95]]),
            feature_incompat: u32::from_le_bytes([bin[96], bin[97], bin[98], bin[99]]),
            feature_ro_compat: u32::from_le_bytes([bin[100], bin[101], bin[102], bin[103]]),
            uuid: u128::from_le_bytes([bin[104], bin[105], bin[106], bin[107], bin[108], bin[109], bin[110], bin[111], bin[112], bin[113], bin[114], bin[115], bin[116], bin[117], bin[118], bin[119]]),
            volume_name: [bin[120], bin[121], bin[122], bin[123], bin[124], bin[125], bin[126], bin[127], bin[128], bin[129], bin[130], bin[131], bin[132], bin[133], bin[134], bin[135]],
            last_mounted: bin[136..200].try_into().unwrap(),
            algo_bitmap: u32::from_le_bytes([bin[200], bin[201], bin[202], bin[203]]),
            prealloc_blocks: i8::from_le_bytes([bin[204]]),
            prealloc_dir_blocks: i8::from_le_bytes([bin[205]]),
            journal_uuid: u16::from_le_bytes([bin[206], bin[207]]),
            journal_inum: u32::from_le_bytes([bin[208], bin[209], bin[210], bin[211]]),
            journal_dev: u32::from_le_bytes([bin[212], bin[213], bin[214], bin[215]]),
            last_orphan: u32::from_le_bytes([bin[216], bin[217], bin[218], bin[219]]),
            hash_seed: [u32::from_le_bytes([bin[220], bin[221], bin[222], bin[223]]), u32::from_le_bytes([bin[224], bin[225], bin[226], bin[227]]), u32::from_le_bytes([bin[228], bin[229], bin[230], bin[231]]), u32::from_le_bytes([bin[232], bin[233], bin[234], bin[235]])],
            def_hash_version: u8::from_le_bytes([bin[236]]),
            default_mount_options: u32::from_le_bytes([bin[237], bin[238], bin[239], bin[240]]),
            first_meta_bg: u32::from_le_bytes([bin[241], bin[242], bin[243], bin[244]]),
        }
    }

}
