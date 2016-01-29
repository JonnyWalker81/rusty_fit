use std::io::Read;
use std::fs;
use std::path::Path;

struct Decoder {
    file_name: String,
    fit_buf: Vec<u8>
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new()
        }
    } 

    pub fn decode(&mut self) {
        self.fit_buf = read_bin(&self.file_name);

        /*for w in &self.fit_buf {
            println!("{}", w);
        }
        */

        let header = self.read_header();
        println!("Header: {:?}", header);

        let header_size = header[0];
        println!("Header Size: {}", header_size);

        let protocol_version = header[1];
        println!("Protocol Version: {}", protocol_version);

        let protocol_profile_major = header[2];
        println!("Protocol Profile Major: {}", protocol_profile_major);

        let protocol_profile_minor = header[3];
        println!("Protocol Profile Minor: {}", protocol_profile_minor);

        let full_profile_version: u16 = (*header[3] as u16) << 8 | (*header[2] as u16);
        println!("Protocol Profile Version: {}", full_profile_version);

        let record_size_in_bytes: u32 = (*header[7] as u32) << 24 | 
            (*header[6] as u32) << 16 | 
            (*header[5] as u32) << 8 | 
            (*header[4] as u32);
        println!("Record Size: {}", record_size_in_bytes);

        let mut fit_str = String::new();
        fit_str.push(*header[8] as char);
        fit_str.push(*header[9] as char);
        fit_str.push(*header[10] as char);
        fit_str.push(*header[11] as char);
        println!("{}", fit_str);

        let crc: u16 = (*header[13] as u16) << 8 | (*header[12] as u16);
        println!("CRC: {}", crc);
    }

    fn read_header(&self) -> Vec<&u8> {
        let word0 = self.fit_buf[0];
        let word1 = self.fit_buf[1];

        let headerVec: Vec<&u8> = self.fit_buf.iter().take(14).collect();

        return headerVec;
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
    println!("hello");
}
