use std::collections::HashMap;
use super::type_table::TypeTableEntry;

pub struct GlobalMessage {
    table: HashMap<u16, &'static str>
}

impl TypeTableEntry for GlobalMessage {
    fn get(&self, key: u16) -> &'static str {
        let result = self.table.get(&key);

        match result {
            Some(r) => return r,
            None => panic!("Unregcognized Global Message Number: {}", key)
        };
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
