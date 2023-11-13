use std::io::{self, Seek, Read};

use std::fs::File;

fn process_file(path: &str) -> Result<(), io::Error>
{
    let mut file = File::open(path)?;
    let len = file.seek(io::SeekFrom::End(0))?;
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
    file.seek(io::SeekFrom::Start(0))?;
    file.read_to_end(&mut buf)?;
    // for i in 0..buf.len() {
    //     println!("{}, {}", buf[i], buf[i] as char);
    // }
    Ok(())
}

fn main()
{
    if let Err(err) = process_file("test.md") {
        println!("Failed to process file, {:?}", err);
    }
}
