extern crate byteorder;
extern crate chrono;
use std::io::Read;
use std::fs;
use std::path::Path;
use byteorder::{ByteOrder, BigEndian, LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use chrono::*;
mod decoder;
mod fit_header;
mod global_message_table;
mod file_id_message;
mod field_definition;
mod fit_file;

pub fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

#[test]
fn it_works() {
    let mut decoder = decoder::Decoder::new(String::from("./test_files/1025146781.fit"));
    decoder.decode();
}
