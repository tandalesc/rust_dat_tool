use std::io::Result;

use super::DatFileReader;
use super::file::DatFile;

const BYTE_SEQ_RECORDING_SPEED: [u8; 4] = [0x30, 0x30, 0x07, 0x30];
const BYTE_SEQ_DATE_TIME: [u8; 4] = [0x40, 0x40, 0x0E, 0x40];
const BYTE_SEQ_IMAGE_NUM: [u8; 4] = [0x60, 0x60, 0x05, 0x60];
const BYTE_SEQ_IMAGES: [u8; 4] = [0xA0, 0xA0, 0x01, 0xA0];

#[derive(Default)]
pub struct DatFileParser {}

impl DatFileParser {
    pub fn process(mut reader: DatFileReader) -> Result<DatFile> {
        reader.seek_until(&BYTE_SEQ_RECORDING_SPEED)?;
        reader.skip(10)?;
        let recording_speed_str = reader.read_string(3)?;
        let recording_speed_u32 = recording_speed_str.parse().unwrap();

        reader.seek_until(&BYTE_SEQ_DATE_TIME)?;
        reader.skip(10)?;
        let dt = reader.read_date_time()?;

        reader.seek_until(&BYTE_SEQ_IMAGE_NUM)?;
        reader.skip(10)?;
        let image_num = reader.read_int()?;

        reader.seek_until(&BYTE_SEQ_IMAGES)?;
        reader.skip(10)?;
        let images_u8 = reader.read_until_end()?;

        Ok(DatFile {
            recording_speed: recording_speed_u32,
            date_time: dt,
            image_num: image_num as u32,
            image_data: images_u8,
        })
    }
}
