use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::str::from_utf8;

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let stat = file.metadata()?;

    let mut buffer = vec![0; stat.len() as usize];
    file.read(&mut buffer)?;
    let value = from_utf8(&buffer)?.to_string();

    Ok(value)
}

fn write_file(path: &str, data: &str) -> Result<(), Box<dyn Error>> {
    File::create(path)?.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let hello_world = format!(
        "{}, {}",
        read_file("hello.txt").unwrap(),
        read_file("world.txt").unwrap()
    );
    write_file("hello-world.txt", &hello_world).unwrap();
}
