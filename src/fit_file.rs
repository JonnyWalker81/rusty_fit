use super::fit_header::FitHeader;

pub struct FitFile {
    pub header: FitHeader
}

impl FitFile {
    pub fn new() -> FitFile {
        return FitFile {
            header: FitHeader::default()
        }
    }
}
