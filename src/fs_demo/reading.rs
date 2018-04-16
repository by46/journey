use std::io;
use std::io::prelude::*;
use std::fs;

fn foo() -> io::Result<()> {
    let mut f = fs::File::open("foo.txt")?;
    let mut buff = [0; 10];
    f.read(&mut buff)?;
    println!("The bytes: {:?}", buff);
    Ok(())
}

fn read_last_4() -> io::Result<()> {
    let mut f = fs::File::open("foo.txt")?;
    let mut buff = [0; 5];
    f.seek(io::SeekFrom::End(-5))?;
    f.read(&mut buff)?;
    println!("the bytes: {:?}", buff);
    Ok(())
}

fn read_line() -> io::Result<()> {
    let mut f = fs::File::open("foo.txt")?;
    let mut reader = io::BufReader::new(f);
    let mut buff = String::new();
    reader.read_line(&mut buff)?;

    println!("the line from buf : {:?}", buff);
    Ok(())
}

fn write_byte() -> io::Result<()> {
    let mut f = fs::File::create("foo2.txt")?;
    {
        let mut write = io::BufWriter::new(f);
        write.write(&[42])?;
    }
    Ok(())
}

fn read_iterator() -> io::Result<()> {
    let mut f = fs::File::open("Cargo.toml")?;
    let reader = io::BufReader::new(f);
    for line in reader.lines() {
        println!("line length: {}", line?.len());
    }
    Ok(())
}

pub fn demo() {
    foo();
    read_last_4();
    read_line();
    write_byte();
    read_iterator();
}