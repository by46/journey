use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std;
use std::io;
use std::fs::OpenOptions;

fn read() -> io::Result<()> {
    let mut file = File::open("src/fs_demo/error.log")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    assert_eq!(buff, "hello world");
    Ok(())
}

fn write() -> io::Result<()> {
    let mut file = File::create("src/fs_demo/error.log")?;
    file.write_all(b"hello world")?;
    Ok(())
}

fn read_with_buff() -> io::Result<()> {
    let mut file = File::open("Cargo.lock")?;
    let mut reader = BufReader::new(file);
    let mut buff = String::new();
    reader.read_to_string(&mut buff)?;
    println!("content length {}", buff.len());
    Ok(())
}

fn open_by_open_options() -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).open("Cargo.lock")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    println!("content length {}", buff.len());
    Ok(())
}

fn meta_demo() -> io::Result<()> {
    let mut file = File::open("Cargo.lock")?;
    let meta = file.metadata()?;
    println!("meta {:?}", meta);
    println!("is dir {} {}", meta.is_dir(), meta.is_file());
    println!("file type {:?}", meta.file_type());
    Ok(())
}

fn visit_dir(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path);
            } else {
                println!("file {:?}", path.file_name());
            }
        }
    }
    Ok(())
}

fn read_dir_error(s: &str) -> io::Result<()> {
    let result = fs::read_dir(Path::new(s));
    match result {
        Err(err) => println!("error {}", err),
        Ok(result) => println!("result {:?}", result)
    }
    Ok(())
}

fn create_dir_demo() {
    fs::create_dir_all("target/demo/home");
    fs::remove_dir("target/demo/home");
    fs::rename("target/demo", "target/demo2");
    fs::remove_dir("target/demo2");
}

pub fn demo() {
    write();
    read();
    read_with_buff();
    open_by_open_options();
    meta_demo();
    visit_dir(Path::new("src/fs_demo"));
    visit_dir(Path::new("src/fs_demo/basic.rs"));
    read_dir_error("src/fs_demo/basic.rs");
    create_dir_demo();
}