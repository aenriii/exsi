use std::io::Read;


#[derive(Debug)]

pub struct BlockBitmap {
    // byte is b7 b6 b5 b4 b3 b2 b1 b0
    pub data: Vec<u8>
}
// TODO: find_empty_of_size, allocate_range, deallocate_range
impl BlockBitmap {
    pub fn is_taken(&self, block_id: u16) -> bool {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            return (self.data[req_bit] & 1 << block_id % 8) != 0
        } else {
            panic!("BAD BLOCK ID ON taken() {}", block_id)
        }
    }
    pub fn take(&mut self, block_id: u16) {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            if self.data[req_bit] & 1 << block_id & 8 == 0 {
                self.data[req_bit] |= 1 << block_id & 8;
            }
            else {
                panic!("REALLOCATION OF TAKEN BLOCK {}", block_id)
            }
        } else {
            panic!("BAD BLOCK ID ON take() {}", block_id)
        }
    }
    pub fn release(&mut self, block_id: u16) {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            if self.data.get(req_bit).unwrap() & 1 << block_id & 8 != 0 {
                self.data[req_bit] ^= 1 << block_id & 8;

            }
            else {
                panic!("DEALLOCATION OF UNTAKEN BLOCK {}", block_id)
            }
        } else {
            panic!("BAD BLOCK ID ON release() {}", block_id)
        }
    }
}
impl crate::lib::traits::IntoRaw for BlockBitmap {
    fn into_raw(&self) -> Box<&[u8]> {
        return Box::new(self.data.as_slice()); // borrow checker no skill issue
    }
}
impl crate::lib::traits::FromBin for BlockBitmap {
    fn read_from_bin(bin: &[u8]) -> Self {
        return BlockBitmap {
            data: bin.to_vec()
        }
    }
}
impl crate::lib::traits::ReadFrom for BlockBitmap {
    fn read(reader: &mut std::io::BufReader<std::fs::File>, block_size: u32, superblock: &super::superblock::Superblock) -> Self {
        let mut data = vec![0; block_size as usize];
        reader.read_exact(&mut data).unwrap();
        return BlockBitmap {
            data: data
        }
    }
}