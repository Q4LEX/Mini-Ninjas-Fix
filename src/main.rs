use std::{fs::OpenOptions};
use std::io::{Seek, SeekFrom, Write};

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("ninja.exe")
        .unwrap();

    file.seek(SeekFrom::Start(0x4BC56)).unwrap();
    file.write(&[0x75]).unwrap();
}
