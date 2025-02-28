use std::io::Result;

use super::DatFileReader;
use super::file::DatFile;

const BYTE_SEQ_RECORDING_SPEED: [u8; 4] = [0x30, 0x30, 0x07, 0x30];
const BYTE_SEQ_DATE_TIME: [u8; 4] = [0x40, 0x40, 0x0E, 0x40];
const BYTE_SEQ_IMAGE_NUM: [u8; 4] = [0x60, 0x60, 0x05, 0x60];
const BYTE_SEQ_IMAGES: [u8; 4] = [0xA0, 0xA0, 0x01, 0xA0];

pub struct DatFileParser {
    reader: DatFileReader,
}

impl DatFileParser {
    pub fn from_reader(reader: DatFileReader) -> Result<Self> {
        Ok(Self { reader })
    }

    pub fn scan_dat_file(&mut self) -> Result<DatFile> {
        self.reader.seek_until(&BYTE_SEQ_RECORDING_SPEED)?;
        self.reader.skip(10)?;
        let recording_speed_str = self.reader.read_string(3)?;
        let recording_speed_u32 = recording_speed_str.parse().unwrap();

        self.reader.seek_until(&BYTE_SEQ_DATE_TIME)?;
        self.reader.skip(10)?;
        let dt = self.reader.read_date_time()?;

        self.reader.seek_until(&BYTE_SEQ_IMAGE_NUM)?;
        self.reader.skip(10)?;
        let image_num = self.reader.read_int()?;

        self.reader.seek_until(&BYTE_SEQ_IMAGES)?;
        self.reader.skip(10)?;

        let images_u8 = self.reader.read_until_end()?;

        let dat = DatFile {
            recording_speed: recording_speed_u32,
            date_time: dt,
            image_num: image_num as u32,
            image_data: images_u8,
        };
        Ok(dat)
    }
}
