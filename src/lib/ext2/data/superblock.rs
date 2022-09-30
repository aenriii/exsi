use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom, Read};

#[derive(Debug)]
pub struct Superblock {
    pub inodes_count: i32, // verify: <= inodes_per_group * number of block groups
    pub blocks_count: i32,
    pub r_blocks_count: i32, // blocks restricted to superusers
    pub free_blocks_count: i32,
    pub free_inodes_count: i32,
    pub first_data_block: i32, // id of first data block
    pub log_block_size: i32, // verify: >= 0; block_size = 1024 << log_block_size
    pub log_frag_size: i32,
    pub blocks_per_group: i32,
    pub frags_per_group: i32,
    pub inodes_per_group: i32,
    pub mtime: i32, //last mount
    pub wtime: i32, //last write
    pub mnt_count: i16, //times mounted since last fsck
    pub max_mnt_count: i16, //max times mounted since last fsck
    pub magic: i16, // verify: EXT2_SUPER_MAGIC == 0xEF53
    pub state: i16, // verify: == 1 || 2, VALID | ERROR
    pub errors: i16,
    pub minor_rev_level: i16,
    pub lastcheck: i32, // POSIX unix time of last fsck
    pub checkinterval: i32, // POSIX max unix interval between fscks
    pub creator_os: i32,
    pub rev_level: i32,
    pub def_resuid: i16,
    pub def_resgid: i16,
    pub first_ino: i32,
    pub inode_size: i16,
    pub block_group_nr: i16,
    pub feature_compat: i32,
    pub feature_incompat: i32,
    pub feature_ro_compat: i32,
    pub uuid: u128,
    pub volume_name: [u8; 16],
    pub last_mounted: [u8; 64],
    pub algo_bitmap: i32,
    pub prealloc_blocks: i8,
    pub prealloc_dir_blocks: i8,
    pub journal_uuid: i16,
    pub journal_inum: i32,
    pub journal_dev: i32,
    pub last_orphan: i32,
    pub hash_seed: [i32; 4],
    pub def_hash_version: u8,
    pub default_mount_options: u32,
    pub first_meta_bg: u32
}
impl Superblock {
    pub fn block_size(&mut self) -> i32 {
        1024 << self.log_block_size
    }
    pub fn frag_size(&mut self) -> i32 {
        if (self.log_frag_size < 0) {
            1024 << self.log_frag_size
        } else {
            1024 >> -self.log_frag_size
        }
    }

    pub fn read_from_bin(bin: [u8; 1024]) -> Self {
        todo!()
    }

}
