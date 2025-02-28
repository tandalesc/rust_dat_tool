use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Result, Seek};

use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime};

const BYTE_SEQ_LEN: usize = 4;

pub struct DatFileReader {
    reader: BufReader<File>,
    pointer: i64,
}

impl DatFileReader {
    pub fn new(file_name: String) -> Result<Self> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        Ok(DatFileReader { reader, pointer: 0 })
    }

    pub fn skip(&mut self, bytes_to_skip: usize) -> Result<()> {
        self.reader.seek_relative(bytes_to_skip as i64)?;
        self.pointer += bytes_to_skip as i64;
        Ok(())
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; len];
        let bytes_read = self.reader.read(&mut buffer)?;
        if bytes_read != len {
            println!("got only {} bytes, expected {} bytes", bytes_read, len);
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Failed to read all bytes",
            ));
        }
        self.pointer += len as i64;
        Ok(buffer)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String> {
        let buffer = self.read_bytes(len)?;
        let string = String::from_utf8(buffer).expect("Failed to decode string");
        Ok(string)
    }

    pub fn read_until_end(&mut self) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![];
        self.reader.read_to_end(&mut buffer)?;
        self.pointer += buffer.len() as i64;
        Ok(buffer)
    }

    // reads next 4 bytes as a i32
    pub fn read_int(&mut self) -> Result<i32> {
        let bytes = self.read_bytes(4)?;
        let int = (&bytes[..]).read_i32::<LittleEndian>()?;
        Ok(int)
    }

    // reads next 16 bytes as a date
    pub fn read_date_time(&mut self) -> Result<NaiveDateTime> {
        let bytes = self.read_bytes(16)?;
        let mut cursor = Cursor::new(bytes);
        let year = cursor.read_i16::<LittleEndian>()?;
        let month = cursor.read_u16::<LittleEndian>()?;
        // skip 2 bytes; possibly stores day of week
        cursor.seek_relative(2)?;
        let day = cursor.read_u16::<LittleEndian>()?;
        let hour = cursor.read_u16::<LittleEndian>()?;
        let minute = cursor.read_u16::<LittleEndian>()?;
        let second = cursor.read_u16::<LittleEndian>()?;
        let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
        let dt = date
            .and_hms_opt(hour as u32, minute as u32, second as u32)
            .unwrap();
        Ok(dt)
    }

    // seeks until a specific byte sequence is found
    pub fn seek_until(&mut self, target: &[u8; BYTE_SEQ_LEN]) -> Result<()> {
        // skip until consumes the first byte of the target sequence
        let bytes_read = self.reader.skip_until(target[0])?;
        self.pointer += bytes_read as i64;
        // search for remaining three bytes
        let mut buffer: [u8; BYTE_SEQ_LEN - 1] = [0; BYTE_SEQ_LEN - 1];
        let bytes_read = self.reader.read(&mut buffer)?;
        self.pointer += bytes_read as i64;
        // exit out if we can't read three bytes
        // means we are probably at the end of the file
        if bytes_read != BYTE_SEQ_LEN - 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Failed to read target bytes",
            ));
        }
        // if match, exit, otherwise keep searching
        if buffer == &target[1..] {
            Ok(())
        } else {
            self.seek_until(target)
        }
    }
}
