mod dat;

use std::io::Result;

use dat::{DatFileParser, DatFileReader};

const FILE_NAME: &str = "ExampleDAT_BacklitImpact.dat";
const TIFF_DIR: &str = "tiff_files";

fn main() -> Result<()> {
    let file_name = String::from(FILE_NAME);
    let tiff_dir = String::from(TIFF_DIR);

    let reader = DatFileReader::from_file(file_name)?;
    let mut parser = DatFileParser::from_reader(reader)?;
    let dat = parser.scan_dat_file()?;

    dat.write_tiff_images(tiff_dir)?;
    dat.print_statistics();

    Ok(())
}
