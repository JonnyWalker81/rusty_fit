
#[derive(Debug)]
pub struct FitHeader {
    pub size: u8,
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32,
    pub fit_str: String,
    pub crc: u16
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
