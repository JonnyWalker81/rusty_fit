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
mod file_id_table;
mod field_definition;
mod fit_file;
mod record_content;
mod record_header;
mod data_message;
mod definition_message;
mod record_datum;
mod local_type_table;
mod field_table;
mod type_table;
mod event_table;
mod file_creator_table;
mod event_message_table;
mod event_type_table;
mod device_info_table;
mod field_table_placeholder;

pub fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    println!("Reading .FIT File...");
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    println!("Finishied Reading .FIT File...");
    file_buf
}

#[test]
fn it_works() {
    let mut decoder = decoder::Decoder::new(String::from("./test_files/1025146781.fit"));
    decoder.decode();
}
