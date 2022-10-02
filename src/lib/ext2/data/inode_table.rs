use std::io::Read;

use crate::lib::traits::FromBin;



pub type InodeTable = Vec<Inode>;

#[derive(Debug)]
pub struct Inode {
    pub mode: u16,
    pub uid: u16,
    pub size: u32,
    pub atime: u32,
    pub ctime: u32,
    pub mtime: u32,
    pub dtime: u32,
    pub gid: u16,
    pub links_count: u16,
    pub blocks: u32,
    pub flags: u32,
    pub osd1: u32,
    pub block: [[u8; 15]; 4], // 15 bytes * 4 == 4 bytes * 15
    pub generation: u32,
    pub file_acl: u32,
    pub dir_acl: u32,
    pub faddr: u32,
    pub osd2: [u32; 3] // 12 bytes
}

impl crate::lib::traits::ReadFrom for InodeTable {
    fn read(reader: &mut std::io::BufReader<std::fs::File>, block_size: u32, superblock: &super::superblock::Superblock) -> Self {
        let mut inodes = Vec::new();
        for x in 1..superblock.inodes_per_group {
            let mut buf = [0 as u8; 128];
            reader.read_exact(&mut buf).unwrap();
            let inode = Inode::read_from_bin(&buf);
            inodes.push(inode);
        }
        inodes
    }
}

impl crate::lib::traits::FromBin for Inode {
    fn read_from_bin(bin: &[u8]) -> Self {
        Self {
            mode: u16::from_le_bytes([bin[0], bin[1]]),
            uid: u16::from_le_bytes([bin[2], bin[3]]),
            size: u32::from_le_bytes([bin[4], bin[5], bin[6], bin[7]]),
            atime: u32::from_le_bytes([bin[8], bin[9], bin[10], bin[11]]),
            ctime: u32::from_le_bytes([bin[12], bin[13], bin[14], bin[15]]),
            mtime: u32::from_le_bytes([bin[16], bin[17], bin[18], bin[19]]),
            dtime: u32::from_le_bytes([bin[20], bin[21], bin[22], bin[23]]),
            gid: u16::from_le_bytes([bin[24], bin[25]]),
            links_count: u16::from_le_bytes([bin[26], bin[27]]),
            blocks: u32::from_le_bytes([bin[28], bin[29], bin[30], bin[31]]),
            flags: u32::from_le_bytes([bin[32], bin[33], bin[34], bin[35]]),
            osd1: u32::from_le_bytes([bin[36], bin[37], bin[38], bin[39]]),
            block: [
                bin[40..55].try_into().unwrap(),
                bin[55..70].try_into().unwrap(),
                bin[70..85].try_into().unwrap(),
                bin[85..100].try_into().unwrap()
            ],
            generation: u32::from_le_bytes([bin[100], bin[101], bin[102], bin[103]]),
            file_acl: u32::from_le_bytes([bin[104], bin[105], bin[106], bin[107]]),
            dir_acl: u32::from_le_bytes([bin[108], bin[109], bin[110], bin[111]]),
            faddr: u32::from_le_bytes([bin[112], bin[113], bin[114], bin[115]]),
            osd2: [
                u32::from_le_bytes([bin[116], bin[117], bin[118], bin[119]]),
                u32::from_le_bytes([bin[120], bin[121], bin[122], bin[123]]),
                u32::from_le_bytes([bin[124], bin[125], bin[126], bin[127]])
            ]
        }
    }
}