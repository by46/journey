use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std;
use std::fs::OpenOptions;

fn read() -> std::io::Result<()> {
    let mut file = File::open("src/fs_demo/error.log")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    assert_eq!(buff, "hello world");
    Ok(())
}

fn write() -> std::io::Result<()> {
    let mut file = File::create("src/fs_demo/error.log")?;
    file.write_all(b"hello world")?;
    Ok(())
}

fn read_with_buff() -> std::io::Result<()> {
    let mut file = File::open("Cargo.lock")?;
    let mut reader = BufReader::new(file);
    let mut buff = String::new();
    reader.read_to_string(&mut buff)?;
    println!("content length {}", buff.len());
    Ok(())
}

fn open_by_open_options() -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).open("Cargo.lock")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    println!("content length {}", buff.len());
    Ok(())
}

fn meta_demo() -> std::io::Result<()> {
    let mut file = File::open("Cargo.lock")?;
    let meta = file.metadata()?;
    println!("meta {:?}", meta);
    println!("is dir {} {}", meta.is_dir(), meta.is_file());
    println!("file type {:?}", meta.file_type());
    Ok(())
}

pub fn demo() {
    write();
    read();
    read_with_buff();
    open_by_open_options();
    meta_demo();
}