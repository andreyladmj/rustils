use std::fs::File;
use std::io::Read;

pub fn read_bin(filename: String) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = vec![];
    f.read_to_end(&mut buffer).unwrap();
    buffer
}