use std::fs::{DirBuilder, File};
use std::io::Result;

use byteorder::{LittleEndian, ReadBytesExt};
use chrono::NaiveDateTime;
use tiff::encoder::{TiffEncoder, colortype};

const IMG_WIDTH: usize = 400;
const IMG_HEIGHT: usize = 250;

pub struct DatFile {
    pub date_time: NaiveDateTime,
    pub recording_speed: u32,
    pub image_num: u32,
    pub image_data: Vec<u8>,
}

impl DatFile {
    pub fn print_statistics(&self) {
        println!("Date and Time: {}", self.date_time);
        println!("Recording Speed: {} Hz", self.recording_speed);
        println!("Image Number: {}", self.image_num);
        println!("Image Data Length: {}", self.image_data.len());
    }

    pub fn write_tiff_images(&self, tiff_dir: String) -> Result<()> {
        DirBuilder::new().recursive(true).create(tiff_dir.clone())?;
        let mut idx = 0;
        for mut image_u8 in self.image_data.chunks(2 * IMG_HEIGHT * IMG_WIDTH) {
            let mut image_u16 = [0u16; 1 * IMG_HEIGHT * IMG_WIDTH];
            image_u8
                .read_u16_into::<LittleEndian>(&mut image_u16)
                .unwrap();
            let path = format!("{}/test_output_file_{}.tiff", tiff_dir.clone(), idx);
            idx += 1;

            let mut output_file = File::create(path)?;
            let mut tiff = TiffEncoder::new(&mut output_file).unwrap();
            tiff.write_image::<colortype::Gray16>(IMG_WIDTH as u32, IMG_HEIGHT as u32, &image_u16)
                .unwrap();
        }
        Ok(())
    }
}