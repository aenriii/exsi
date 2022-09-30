



pub type DataBlock = Vec<u8>;

impl super::super::super::traits::IntoRaw for DataBlock {
    fn into_raw(&self) -> Box<&[u8]>{
        return Box::new(self.as_slice())
    }
}