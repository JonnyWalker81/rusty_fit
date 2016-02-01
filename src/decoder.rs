extern crate byteorder;
extern crate chrono;
use std::io::Read;
use std::fs;
use std::path::Path;
use byteorder::{ByteOrder, BigEndian, LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use chrono::*;
use super::fit_header::FitHeader;
use super::global_message_table::GlobalMessage;
use super::file_id_message::FileIdMessage;
use super::field_definition::FieldDefinition;

pub struct Decoder {
    pub file_name: String,
    pub fit_buf: Vec<u8>,
    pub header: FitHeader,
    pub global_mesage_table: GlobalMessage,
    pub file_id_table: FileIdMessage,
    read_pos: u64,
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new(),
            header: FitHeader::default(),
            global_mesage_table: GlobalMessage::new(),
            file_id_table: FileIdMessage::new(),
            read_pos: 0,
        }
    } 

    pub fn decode(&mut self) {
        self.fit_buf = ::read_bin(&self.file_name);

        self.decode_header();
        self.decode_records();
    }

    fn decode_header(&mut self) {
        let header = self.read_header();

        let header_size = header[0];

        let protocol_version = header[1];

        let protocol_profile_major = header[2];

        let protocol_profile_minor = header[3];

        let full_profile_version: u16 = (header[3] as u16) << 8 | (header[2] as u16);

        let record_size_in_bytes: u32 = (header[7] as u32) << 24 | 
            (header[6] as u32) << 16 | 
            (header[5] as u32) << 8 | 
            (header[4] as u32);

        let mut fit_str = String::new();
        fit_str.push(header[8] as char);
        fit_str.push(header[9] as char);
        fit_str.push(header[10] as char);
        fit_str.push(header[11] as char);

        let crc: u16 = (header[13] as u16) << 8 | (header[12] as u16);

       self.header.size = header_size; 
       self.header.protocol_version = protocol_version;
       self.header.profile_version = full_profile_version;
       self.header.fit_str = fit_str;
       self.header.data_size = record_size_in_bytes;
       self.header.crc = crc;

       println!("{:?}", self.header);
    }

    fn read_header(&self) -> Vec<u8> {
        let word0 = self.fit_buf[0];
        let word1 = self.fit_buf[1];

        let headerVec: Vec<u8> = self.fit_buf.iter().take(14).map(|b| *b).collect();

        return headerVec;
    }

    fn decode_records(&self) {
        let record_header = self.fit_buf[14];
        println!("{:#b}", record_header);
        let arch = self.fit_buf[16];
        println!("{:#b}", arch);

        let global_message_number: u16 = (self.fit_buf[18] as u16) << 8 | (self.fit_buf[17] as u16);
        println!("Global Message Number: {}", global_message_number);

        let global_message = self.global_mesage_table.get(global_message_number);
        println!("Global Message: {}", global_message);

        let num_fields = self.fit_buf[19];
        println!("Number of Fields: {}", num_fields);

        let index: usize = 20;
        let fields: usize = num_fields as usize;
        let mut pos = index;
        for f in 0..fields {
            let current_index: usize = index + (3 * f);
            let field_def_num = self.fit_buf[current_index];
            let size = self.fit_buf[current_index + 1];
            let base_type = self.fit_buf[current_index + 2];

            let field_kind = self.file_id_table.get(field_def_num);

            println!("{}: {}  {}  {}", field_kind, field_def_num, size, base_type);
            pos += 3;
        }

        //TODO: Handle pos better
        let data_header = self.fit_buf[pos];
        pos += 1;
        println!("{:#b}", data_header);

        let serial_number = LittleEndian::read_u32(&self.fit_buf[pos..]);
        println!("Serial Number: {}", serial_number);
        pos += 4;

        let time_created = LittleEndian::read_u32(&self.fit_buf[pos..]);
        println!("Time Created: {}", time_created);

        let d = UTC.ymd(1989, 12, 31);
        let sum = d + Duration::seconds(time_created as i64);
        println!("Date: {}", sum);
        pos += 4;

        let manufacturer = LittleEndian::read_u16(&self.fit_buf[pos..]);
        println!("Manf: {}", manufacturer);
        pos += 2;

        let product = LittleEndian::read_u16(&self.fit_buf[pos..]);
        println!("Product: {}", product);
        pos += 2;

        let t = self.fit_buf[pos];
        println!("Type: {}", t);
    }

    fn read_u8(&mut self) -> u8 {
        let byte = self.fit_buf[self.read_pos as usize];
        self.read_pos += 1;
        byte
    }

    fn read_byte(&mut self) -> u8{
        let byte = self.fit_buf[self.read_pos as usize];
        self.read_pos += 1;
        byte
    }

    fn read_bytes(&mut self, amount: u32) -> Vec<u8> {
        let mut result = Vec::new();
        for _ in 0..amount {
            let read0 = self.fit_buf[self.read_pos as usize];
            result.push(read0);
            self.read_pos += 1;
        }

        result
    }
}

/*fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
*/
