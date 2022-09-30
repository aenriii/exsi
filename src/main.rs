
mod lib;
use crate::lib::ext2::read::Ext2Image;
fn main() {
    let img = Ext2Image::new(std::env::args().nth(1).expect("No file provided").as_str());
}
