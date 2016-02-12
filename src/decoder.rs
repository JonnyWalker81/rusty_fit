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
use super::file_id_table::FileIdTable;
use super::field_definition::FieldDefinition;
use super::fit_file::FitFile;
use super::record_content::{RecordContent, RecordData};
use super::record_header::RecordHeader;
use super::definition_message::DefinitionMessage;
use super::data_message::DataMessage;
use super::record_datum::RecordDatum;
use super::local_type_table::LocalMessageTable;
use super::field_table::{FieldTable, FieldTableEntry};
use super::type_table::{TypeTable, TypeTableEntry};
use std::rc::Rc;

pub struct Decoder {
    pub file_name: String,
    pub fit_buf: Vec<u8>,
    //pub header: FitHeader,
    pub global_mesage_table: GlobalMessage,
    // pub file_id_table: FileIdMessage,
    local_message_table: LocalMessageTable,
    field_table: FieldTable,
    type_table: TypeTable,
    read_pos: u64,
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new(),
            //header: FitHeader::default(),
            global_mesage_table: GlobalMessage::new(),
            // file_id_table: FileIdMessage::new(),
            local_message_table: LocalMessageTable::new(),
            type_table: TypeTable::new(),
            field_table: FieldTable::new(),
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
        let header_size = self.fit_buf
                              .first()
                              .unwrap()
                              .clone();
        let header = self.read_header(header_size);

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

    fn read_header(&mut self, header_size: u8) -> Vec<u8> {
        //let word0 = self.fit_buf[0];
        //let word1 = self.fit_buf[1];

        let headerVec: Vec<u8> = self.fit_buf
                                     .iter()
                                     .take(header_size as usize)
                                     .map(|b| *b)
                                     .collect();
        self.read_pos += header_size as u64;

        return headerVec;
    }

    fn decode_records(&mut self) -> RecordContent {
        let mut record_content = RecordContent::new();
        
        for i in 0..8 {
            println!("Loop: {}", i);
            println!("Current Pos: {}", self.read_pos);
            self.decode_message();
            println!("");
            println!("");
        }
        
        record_content
    }

    fn decode_message(&mut self) -> Box<RecordData> {
        let raw_header = self.read_u8();
        println!("Record Header: {:#b}", raw_header);
        let record_header = RecordHeader::new(raw_header);
        println!("RecordHeader: {:?}", record_header);
        if record_header.is_definition() {
            let def_message = self.decode_definition_message(record_header);
            def_message
        }
        else {
            let data_message = self.decode_data_message(record_header);
            data_message
        }
    }

    fn get_table(&self, key: &'static str) -> Rc<TypeTableEntry> {
        let result = self.type_table.get(&key);
        result.clone()
    }

    fn decode_definition_message(&mut self, record_header: RecordHeader) -> Box<DefinitionMessage> {
        println!("Decoding Definition Message...");
        let mut definition_message = DefinitionMessage::new(record_header);
        self.read_u8(); // read and ignore since the next byte is reserved
        let arch = self.read_u8(); //TODO: Handle Endianess better
        println!("Arch: {:#b}", arch);
        definition_message.architecture = arch;

        let mut two_bytes = self.read_bytes(2);
        let global_message_number = LittleEndian::read_u16(&two_bytes);
        println!("Global Message Number: {}", global_message_number);
        definition_message.global_message_number = global_message_number;

        let mut global_table: Rc<TypeTableEntry> = Rc::new(GlobalMessage::new());
        {
            global_table = self.get_table("global");
        }
        let global_message = global_table.get(global_message_number);
        println!("Global Message: {}", global_message);

        let num_fields = self.read_u8();
        println!("Number of Fields: {}", num_fields);
        definition_message.number_of_fields = num_fields;

        // let global_message = self.global_mesage_table.get(global_message_number);
        let global_message = global_table.get(global_message_number);
        // let mut message_mapping: HashMap<i32, &'static str> = HashMap::new();
        let mut message_mapping: Rc<FieldTableEntry> = Rc::new(FileIdTable::new());
        {
            message_mapping = self.field_table.get(global_message);
        }

        for f in 0..num_fields {
            let field_def_num = self.read_u8();
            let size = self.read_u8();
            let base_type = self.read_u8();

            let key = field_def_num as i32;
            let field_kind = message_mapping.get(key);

            let field_definition = FieldDefinition::new(field_def_num, size, base_type);
            definition_message.fields.push(field_definition);

            println!("{}: {:?}", field_kind, field_definition);
        }

        println!("DefinitionMessage: {:?}", definition_message);

        let key = record_header.get_local_type();
        self.local_message_table.insert(key, definition_message.clone());

        Box::new(definition_message)
    }

    fn decode_data_message(&mut self, record_header: RecordHeader) -> Box<DataMessage> {
        println!("Decoding Data Message...");
        //TODO: Handle pos better
        // let data_header = self.read_u8();
        // println!("{:#b}", data_header);

        let mut data_message = DataMessage::new(record_header);
        let local_type = record_header.get_local_type();
        let definition = self.local_message_table.get(local_type);

        let global_number = definition.global_message_number;
        // let global_message = self.global_mesage_table.get(global_number);
        // TODO: Figure out best way to handle this to make borrow cheker happy
        let mut global_table: Rc<TypeTableEntry> = Rc::new(GlobalMessage::new());
        {
            global_table = self.get_table("global");
        }
        let global_message = global_table.get(global_number);
        // let mut message_mapping: HashMap<i32, &'static str> = HashMap::new();
        let mut message_mapping: Rc<FieldTableEntry> = Rc::new(FileIdTable::new());
        {
            println!("Global Message: {}", global_message);
            message_mapping = self.field_table.get(global_message);
        }
        println!("Def: {:?}", definition);
        for f in definition.fields {
            let key = f.field_number as i32;
            // println!("Key: {}", key);
            // println!("Message Mapping: {:?}", message_mapping);
            let field_name = message_mapping.get(key);
            println!("{}", field_name);
            let read_result = match f.size {
                1 => self.read_byte() as u32,
                2 => {
                    let r = self.read_bytes(2);
                    Decoder::get_u16(definition.architecture, &r) as u32
                },
                4 => {
                    let r = self.read_bytes(4);
                    Decoder::get_u32(definition.architecture, &r)
                },
                _ => panic!("Unhandled size: {}", f.size)
            };
            let mut datum = RecordDatum::new(field_name.to_string(), read_result as i64);
            data_message.push_datum(datum);
        }

        /*let mut four_bytes = self.read_bytes(4);
        // let serial_number = LittleEndian::read_u32(&four_bytes);
        let serial_number = Decoder::get_u32(definition.architecture, &four_bytes);
        println!("Serial Number: {}", serial_number);
        let mut datum = RecordDatum::new(String::from("Serial Number"), serial_number as i64);
        data_message.push_datum(datum);

        four_bytes = self.read_bytes(4);
        // let time_created = LittleEndian::read_u32(&four_bytes);
        let time_created = Decoder::get_u32(definition.architecture, &four_bytes);
        println!("Time Created: {}", time_created);
        datum = RecordDatum::new(String::from("Time Created"), time_created as i64);
        data_message.push_datum(datum);

        let d = UTC.ymd(1989, 12, 31).and_hms(0, 0, 0);
        let sum = d + Duration::seconds(time_created as i64);
        println!("Date: {}", sum);
        datum = RecordDatum::new(String::from("Date"), sum.timestamp());
        data_message.push_datum(datum);

        let mut two_bytes = self.read_bytes(2);
        // let manufacturer = LittleEndian::read_u16(&two_bytes);
        let manufacturer = Decoder::get_u16(definition.architecture, &two_bytes);
        println!("Manf: {}", manufacturer);
        datum = RecordDatum::new(String::from("Manf"), manufacturer as i64);
        data_message.push_datum(datum);

        two_bytes = self.read_bytes(2);
        // let product = LittleEndian::read_u16(&two_bytes);
        let product = Decoder::get_u16(definition.architecture, &two_bytes);
        println!("Product: {}", product);
        datum = RecordDatum::new(String::from("Product"), product as i64);
        data_message.push_datum(datum);

        two_bytes = self.read_bytes(2);
        // let number = LittleEndian::read_u16(&two_bytes);
        let number = Decoder::get_u16(definition.architecture, &two_bytes);
        println!("Number: {}", number);
        datum = RecordDatum::new(String::from("Number"), number as i64);
        data_message.push_datum(datum);

        let t = self.read_byte();
        println!("Type: {}", t);
        datum = RecordDatum::new(String::from("Type"), t as i64);
        data_message.push_datum(datum);
        */

        println!("DataMessage: {:?}", data_message);
        
        Box::new(data_message)
    }

    fn get_u32(arch: u8, buf: &[u8]) -> u32 {
        match arch {
            0 => LittleEndian::read_u32(buf),
            1 => BigEndian::read_u32(buf),
            _ => 0
        }
    }

    fn get_u16(arch: u8, buf: &[u8]) -> u16 {
        match arch {
            0 => LittleEndian::read_u16(buf),
            1 => BigEndian::read_u16(buf),
            _ => 0
        }
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
