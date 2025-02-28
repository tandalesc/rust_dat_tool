mod dat;

use std::io::Result;

use dat::{DatFileParser, DatFileReader};

fn main() -> Result<()> {
    let file_name = String::from("ExampleDAT_BacklitImpact.dat");
    let tiff_dir = String::from("tiff_files");

    let reader = DatFileReader::new(file_name)?;
    let mut parser = DatFileParser::from_reader(reader)?;
    let dat = parser.scan_dat_file()?;
    dat.write_tiff_images(tiff_dir)?;
    dat.print_statistics();

    Ok(())
}
