



pub type DataBlock = Vec<u8>;

impl crate::lib::traits::IntoRaw for DataBlock {
    fn into_raw(&self) -> Box<&[u8]>{
        return Box::new(self.as_slice())
    }
}
impl crate::lib::traits::FromBin for DataBlock {
    fn read_from_bin(bin: &[u8]) -> Self {
        return bin.to_vec()
    }
}