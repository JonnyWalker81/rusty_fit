extern crate byteorder;
use std::io::Read;
use std::fs;
use std::path::Path;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

struct Decoder {
    file_name: String,
    fit_buf: Vec<u8>,
    header: FitHeader
}

#[derive(Debug)]
struct FitHeader {
    size: u8,
    protocol_version: u8,
    profile_version: u16,
    data_size: u32,
    fit_str: String,
    crc: u16
}

impl Default for FitHeader {
    fn default() -> FitHeader {
        return FitHeader{
                fit_str: String::new(),
                profile_version: 0,
                protocol_version: 0,
                data_size: 0,
                crc: 0,
                size: 0
            }
    }
}

struct FieldDefinition {
    field_number: u8,
    size: u8,
    base_type_number: u8
}

struct GlobalMessage;

impl GlobalMessage {
    
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new(),
            header: FitHeader::default()
        }
    } 

    pub fn decode(&mut self) {
        self.fit_buf = read_bin(&self.file_name);

        /*for w in &self.fit_buf {
            println!("{}", w);
        }
        */
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

        let num_fields = self.fit_buf[19];
        println!("Number of Fields: {}", num_fields);

        let index: usize = 20;
        let fields: usize = num_fields as usize;
        for f in 0..fields {
            let current_index: usize = index + (3 * f);
            let field_def_num = self.fit_buf[current_index];
            let size = self.fit_buf[current_index + 1];
            let base_type = self.fit_buf[current_index + 2];

            println!("{}  {}  {}", field_def_num, size, base_type);
        }
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

#[test]
fn it_works() {
    let mut decoder = Decoder::new(String::from("./test_files/1025146781.fit"));
    decoder.decode();
}
