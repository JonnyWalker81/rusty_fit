use std::collections::HashMap;
use super::type_table::{TypeTableEntry, TypeTableType};

pub struct GlobalMessage {
    table: TypeTableType
}

impl TypeTableEntry for GlobalMessage {
    fn get(&self, key: u16) -> String {
        let result = self.table.get(&key);

        match result {
            Some(r) => r.clone(),
            None => key.to_string().clone()
        }
    }
}

impl GlobalMessage {
    pub fn new() -> GlobalMessage {
        let global_table = GlobalMessage::make_global_table();
        return GlobalMessage {
            table: global_table
        }
    }

    // pub fn get(&self, key: u16) -> &'static str {
    //     let result = self.table.get(&key);

    //     match result {
    //         Some(r) => return r,
    //         None => panic!("Unregcognized Global Message Number: {}", key)
    //     };
    // }

    fn make_global_table() -> TypeTableType {
        let mut global_table = TypeTableType::new();
        global_table.insert(0, String::from("file_id"));
        global_table.insert(1, String::from("capabilities"));
        global_table.insert(2, String::from("device_settings"));
        global_table.insert(3, String::from("user_profile"));
        global_table.insert(4, String::from("hrm_profile"));
        global_table.insert(5, String::from("sdm_profile"));
        global_table.insert(6, String::from("bike_profile"));
        global_table.insert(7, String::from("zones_target"));
        global_table.insert(8, String::from("hr_zone"));
        global_table.insert(9, String::from("power_zone"));
        global_table.insert(10, String::from("met_zone"));
        global_table.insert(12, String::from("sport"));
        global_table.insert(15, String::from("goal"));
        global_table.insert(18, String::from("session"));
        global_table.insert(19, String::from("lap"));
        global_table.insert(20, String::from("record"));
        global_table.insert(21, String::from("event"));
        global_table.insert(23, String::from("device_info"));
        global_table.insert(26, String::from("workout"));
        global_table.insert(27, String::from("workout_step"));
        global_table.insert(28, String::from("schedule"));
        global_table.insert(30, String::from("weight_scale"));
        global_table.insert(31, String::from("course"));
        global_table.insert(32, String::from("course_point"));
        global_table.insert(33, String::from("totals"));
        global_table.insert(34, String::from("activity"));
        global_table.insert(35, String::from("software"));
        global_table.insert(37, String::from("file_capabilities"));
        global_table.insert(38, String::from("mesg_capabilities"));
        global_table.insert(39, String::from("field_capabilities"));
        global_table.insert(49, String::from("file_creator"));
        global_table.insert(51, String::from("blood_pressure"));
        global_table.insert(53, String::from("speed_zone"));
        global_table.insert(55, String::from("monitoring"));
        global_table.insert(72, String::from("training_file"));
        global_table.insert(78, String::from("hrv"));
        global_table.insert(80, String::from("ant_rx"));
        global_table.insert(81, String::from("ant_tx"));
        global_table.insert(82, String::from("ant_channel_id"));
        global_table.insert(101, String::from("length"));
        global_table.insert(103, String::from("monitoring_info"));
        global_table.insert(105, String::from("pad"));
        global_table.insert(106, String::from("slave_device"));
        global_table.insert(131, String::from("cadance_sonze"));
        global_table.insert(132, String::from("hr"));
        global_table.insert(142, String::from("segment_lap"));
        global_table.insert(145, String::from("memo_glob"));
        global_table.insert(148, String::from("segment_id"));
        global_table.insert(149, String::from("segment_leaderboard_entry"));
        global_table.insert(150, String::from("segment_point"));
        global_table.insert(151, String::from("segment_file"));
        global_table.insert(160, String::from("gps_metadata"));
        global_table.insert(161, String::from("camera_event"));
        global_table.insert(162, String::from("timestamp_correlation"));
        global_table.insert(164, String::from("gyroscope_data"));
        global_table.insert(165, String::from("accelerometer_data"));
        global_table.insert(167, String::from("three_d_sensor_calibration"));
        global_table.insert(169, String::from("video_frame"));
        global_table.insert(174, String::from("obdii_data"));
        global_table.insert(177, String::from("nmea_sentence"));
        global_table.insert(178, String::from("aviation_attitude"));
        global_table.insert(184, String::from("video"));
        global_table.insert(185, String::from("video_title"));
        global_table.insert(186, String::from("video_description"));
        global_table.insert(187, String::from("video_clip"));
        global_table.insert(0xFF00, String::from("mfg_range_min"));
        global_table.insert(0xFFFE, String::from("mfg_range_max"));

        global_table
    }
}
