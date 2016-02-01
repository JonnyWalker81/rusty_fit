extern crate byteorder;
extern crate chrono;
use std::io::Read;
use std::fs;
use std::path::Path;
use byteorder::{ByteOrder, BigEndian, LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use chrono::*;

struct Decoder {
    file_name: String,
    fit_buf: Vec<u8>,
    header: FitHeader,
    global_mesage_table: GlobalMessage,
    file_id_table: FileIdMessage,
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

struct FileIdMessage {
    table: HashMap<u8, &'static str>
}

struct GlobalMessage {
    table: HashMap<u16, &'static str>
}

impl GlobalMessage {
    fn new() -> GlobalMessage {
        let global_table = GlobalMessage::make_global_table();
        return GlobalMessage {
            table: global_table
        }
    }

    fn get(&self, key: u16) -> &'static str {
        let result = self.table.get(&key);

        match result {
            Some(r) => return r,
            None => panic!("Unregcognized Global Message Number: {}", key)
        };
    }

    fn make_global_table() -> HashMap<u16, &'static str> {
        let mut global_table = HashMap::new();
        global_table.insert(0, "file_id");
        global_table.insert(1, "capabilities");
        global_table.insert(2, "device_settings");
        global_table.insert(3, "user_profile");
        global_table.insert(4, "hrm_profile");
        global_table.insert(5, "sdm_profile");
        global_table.insert(6, "bike_profile");
        global_table.insert(7, "zones_target");
        global_table.insert(8, "hr_zone");
        global_table.insert(9, "power_zone");
        global_table.insert(10, "met_zone");
        global_table.insert(12, "sport");
        global_table.insert(15, "goal");
        global_table.insert(18, "session");
        global_table.insert(19, "lap");
        global_table.insert(20, "record");
        global_table.insert(21, "event");
        global_table.insert(23, "device_info");
        global_table.insert(26, "workout");
        global_table.insert(27, "workout_step");
        global_table.insert(28, "schedule");
        global_table.insert(30, "weight_scale");
        global_table.insert(31, "course");
        global_table.insert(32, "course_point");
        global_table.insert(33, "totals");
        global_table.insert(34, "activity");
        global_table.insert(35, "software");
        global_table.insert(37, "file_capabilities");
        global_table.insert(38, "mesg_capabilities");
        global_table.insert(39, "field_capabilities");
        global_table.insert(49, "file_creator");
        global_table.insert(51, "blood_pressure");
        global_table.insert(53, "speed_zone");
        global_table.insert(55, "monitoring");
        global_table.insert(72, "training_file");
        global_table.insert(78, "hrv");
        global_table.insert(80, "ant_rx");
        global_table.insert(81, "ant_tx");
        global_table.insert(82, "ant_channel_id");
        global_table.insert(101, "length");
        global_table.insert(103, "monitoring_info");
        global_table.insert(105, "pad");
        global_table.insert(106, "slave_device");
        global_table.insert(131, "cadance_sonze");
        global_table.insert(132, "hr");
        global_table.insert(142, "segment_lap");
        global_table.insert(145, "memo_glob");
        global_table.insert(148, "segment_id");
        global_table.insert(149, "segment_leaderboard_entry");
        global_table.insert(150, "segment_point");
        global_table.insert(151, "segment_file");
        global_table.insert(160, "gps_metadata");
        global_table.insert(161, "camera_event");
        global_table.insert(162, "timestamp_correlation");
        global_table.insert(164, "gyroscope_data");
        global_table.insert(165, "accelerometer_data");
        global_table.insert(167, "three_d_sensor_calibration");
        global_table.insert(169, "video_frame");
        global_table.insert(174, "obdii_data");
        global_table.insert(177, "nmea_sentence");
        global_table.insert(178, "aviation_attitude");
        global_table.insert(184, "video");
        global_table.insert(185, "video_title");
        global_table.insert(186, "video_description");
        global_table.insert(187, "video_clip");
        global_table.insert(0xFF00, "mfg_range_min");
        global_table.insert(0xFFFE, "mfg_range_max");

        return global_table;
    }
}

impl FileIdMessage {
    fn new() -> FileIdMessage {
        return FileIdMessage {
            table: FileIdMessage::make_table()
        }
    }

    fn get(&self, key: u8) -> &'static str {
        let result = self.table.get(&key);

        match result {
            Some(r) => return r,
            None => panic!("Unrecognized File ID Key: {}", key)
        };
    }

    fn make_table() -> HashMap<u8, &'static str> {
        let mut table = HashMap::new();
        table.insert(0, "type");
        table.insert(1, "manufacturer");
        table.insert(2, "product");
        table.insert(3, "serial_number");
        table.insert(4, "time_created");
        table.insert(5, "number");
        table.insert(7, "product_name");

        return table;
    }
}

impl Decoder {
    pub fn new(file_name: String) -> Decoder {
        return Decoder {
            file_name: file_name,
            fit_buf: Vec::new(),
            header: FitHeader::default(),
            global_mesage_table: GlobalMessage::new(),
            file_id_table: FileIdMessage::new(),
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
