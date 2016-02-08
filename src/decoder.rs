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
use super::fit_file::FitFile;
use super::record_content::{RecordContent, RecordData};
use super::record_header::RecordHeader;
use super::definition_message::DefinitionMessage;

pub struct Decoder {
    pub file_name: String,
    pub fit_buf: Vec<u8>,
    //pub header: FitHeader,
    pub global_mesage_table: GlobalMessage,
    pub file_id_table: FileIdMessage,
    read_pos: u64,
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new(),
            //header: FitHeader::default(),
            global_mesage_table: GlobalMessage::new(),
            file_id_table: FileIdMessage::new(),
            read_pos: 0,
        }
    } 

    pub fn decode(&mut self) -> FitFile {
        let mut fit_file = FitFile::new();
        self.fit_buf = super::read_bin(&self.file_name);

        fit_file.header = self.decode_header();
        println!("{:?}", fit_file.header);
        let record_content = self.decode_records();
        fit_file.data = record_content;

        fit_file
    }

    fn decode_header(&mut self) -> FitHeader {
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

        let mut fit_header = FitHeader::default();
        fit_header.size = header_size; 
        fit_header.protocol_version = protocol_version;
        fit_header.profile_version = full_profile_version;
        fit_header.fit_str = fit_str;
        fit_header.data_size = record_size_in_bytes;
        fit_header.crc = crc;

        //println!("{:?}", fit_header);

        fit_header
    }

    fn read_header(&mut self) -> Vec<u8> {
        //let word0 = self.fit_buf[0];
        //let word1 = self.fit_buf[1];

        let headerVec: Vec<u8> = self.fit_buf.iter().take(14).map(|b| *b).collect();
        self.read_pos += 14;

        return headerVec;
    }

    fn decode_records(&mut self) -> RecordContent {
        let mut record_content = RecordContent::new();
        
        self.decode_message();
        //TODO: Handle pos better
        let data_header = self.read_u8();
        println!("{:#b}", data_header);

        let mut four_bytes = self.read_bytes(4);
        let serial_number = LittleEndian::read_u32(&four_bytes);
        println!("Serial Number: {}", serial_number);

        four_bytes = self.read_bytes(4);
        let time_created = LittleEndian::read_u32(&four_bytes);
        println!("Time Created: {}", time_created);

        let d = UTC.ymd(1989, 12, 31);
        let sum = d + Duration::seconds(time_created as i64);
        println!("Date: {}", sum);

        let mut two_bytes = self.read_bytes(2);
        let manufacturer = LittleEndian::read_u16(&two_bytes);
        println!("Manf: {}", manufacturer);

        two_bytes = self.read_bytes(2);
        let product = LittleEndian::read_u16(&two_bytes);
        println!("Product: {}", product);

        let t = self.read_byte();
        println!("Type: {}", t);

        record_content
    }

    fn decode_message(&mut self) -> Box<RecordData> {
        let raw_header = self.read_u8();
        let record_header = RecordHeader::new(raw_header);
        println!("RecordHeader: {:?}", record_header);
        self.read_u8(); // read and ignore since the next byte is reserved
        println!("{:#b}", raw_header);
        let arch = self.read_u8(); //TODO: Handle Endianess better
        println!("{:#b}", arch);

        let mut two_bytes = self.read_bytes(2);
        let global_message_number = LittleEndian::read_u16(&two_bytes);
        println!("Global Message Number: {}", global_message_number);

        let global_message = self.global_mesage_table.get(global_message_number);
        println!("Global Message: {}", global_message);

        let num_fields = self.read_u8();
        println!("Number of Fields: {}", num_fields);

        for f in 0..num_fields {
            let field_def_num = self.read_u8();
            let size = self.read_u8();
            let base_type = self.read_u8();

            let field_kind = self.file_id_table.get(field_def_num);

            println!("{}: {}  {}  {}", field_kind, field_def_num, size, base_type);
        }

        Box::new(DefinitionMessage::new(record_header))
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
