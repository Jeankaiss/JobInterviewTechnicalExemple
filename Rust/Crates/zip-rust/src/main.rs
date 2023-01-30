extern crate zip;

use std::fs::File;
use std::io::{Seek, Write, Read};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};

// static FILE_CONTENTS: &'static [u8] = include_bytes!("../exemple.txt");
// static FILE_CONTENTS_1: &'static [u8] = include_bytes!("../exemple1.txt");



fn create_zip_archive<T: Seek + Write>(file_name: &mut T) -> ZipResult<()> {

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let mut file = File::open(format!("./exemple.txt")).unwrap();
    file.read_to_end(&mut buffer)?;

    let mut writer = ZipWriter::new(file_name);
    writer.start_file("example.txt", options)?;
    writer.write_all(&*buffer)?;
    buffer.clear();

    writer.finish()?;
    Ok(())
}

fn main() {
    let mut file = File::create("example.zip").expect("Couldn't create file");
    create_zip_archive(&mut file).expect("Couldn't create archive");
}
