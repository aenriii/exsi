
#[derive(Debug)]

pub struct InodeBitmap {
    // byte is b7 b6 b5 b4 b3 b2 b1 b0
    pub data: Vec<u8>
}
// TODO: find_empty_of_size, allocate_range, deallocate_range

impl InodeBitmap {
    pub fn is_taken(&self, block_id: u16) -> bool {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            return (self.data[req_bit] & 1 << block_id % 8) != 0
        } else {
            panic!("BAD INODE ID ON taken() {}", block_id)
        }
    }
    pub fn take(&mut self, block_id: u16) {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            if self.data[req_bit] & 1 << block_id & 8 == 0 {
                self.data[req_bit] |= 1 << block_id & 8;
            }
            else {
                panic!("REALLOCATION OF TAKEN INODE {}", block_id)
            }
        } else {
            panic!("BAD INODE ID ON take() {}", block_id)
        }
    }
    pub fn release(&mut self, block_id: u16) {
        let req_bit = (block_id / 8) as usize;
        if self.data.len() >= req_bit {
            if self.data.get(req_bit).unwrap() & 1 << block_id & 8 != 0 {
                self.data[req_bit] ^= 1 << block_id & 8;

            }
            else {
                panic!("DEALLOCATION OF UNTAKEN INODE {}", block_id)
            }
        } else {
            panic!("BAD INODE ID ON release() {}", block_id)
        }
    }
}
impl crate::lib::traits::IntoRaw for InodeBitmap {
    fn into_raw(&self) -> Box<&[u8]> {
        return Box::new(self.data.as_slice()); // borrow checker no skill issue
    }
}
impl crate::lib::traits::FromBin for InodeBitmap {
    fn read_from_bin(bin: &[u8]) -> Self {
        return InodeBitmap {
            data: bin.to_vec()
        }
    }
}