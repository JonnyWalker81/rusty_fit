use super::fit_header::FitHeader;
use super::record_content::RecordContent;

pub struct FitFile {
    pub header: FitHeader,
    pub data: RecordContent,
    pub crc: u16
}

impl FitFile {
    pub fn new() -> FitFile {
        return FitFile {
            header: FitHeader::default(),
            data: RecordContent::new(),
            crc: 0
        }
    }
}
