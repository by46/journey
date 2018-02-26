use std::env;
use std::fmt;
use std::error;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::num;
use std::num::ParseIntError;
use std::num::ParseFloatError;
use std::result;

use example::common;

type Result<T> = result::Result<T, ParseIntError>;

enum NewOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(ParseIntError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            CliError::ParseInt(ref err) => write!(f, "Parse int error: {}", err),
            CliError::ParseFloat(ref err) => write!(f, "Parse float error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => err.description(),
            CliError::ParseInt(ref err) => err.description(),
            CliError::ParseFloat(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
            CliError::ParseInt(ref err) => Some(err),
            CliError::ParseFloat(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<num::ParseFloatError> for CliError {
    fn from(err: num::ParseFloatError) -> CliError {
        CliError::ParseFloat(err)
    }
}

fn find(haystack: &str, needle: char) -> NewOption<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return NewOption::Some(offset);
        }
    }
    NewOption::None
}

fn find2(haystack: &str, needle: char) -> Option<usize> {
    haystack.find(needle)
}

fn extension_explict(file_name: &str) -> Option<&str> {
    match find2(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..])
    }
}

fn extension_explict2(file_name: &str) -> Option<&str> {
    map(find2(file_name, '.'), |offset| &file_name[offset + 1..])
}

fn extension_explict3(file_name: &str) -> Option<&str> {
    find2(file_name, '.').map(|offset| &file_name[offset + 1..])
}

fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(x) => Some(f(x))
    }
}

impl<T> NewOption<T> {
    fn unwrap(self) -> T {
        match self {
            NewOption::Some(value) => value,
            NewOption::None =>
                panic!("called is None")
        }
    }
}

fn filename(file_path: &str) -> Option<&str> {
    unimplemented!()
}

fn extension(file_name: &str) -> Option<&str> {
    None
}

fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    match filename(file_path) {
        None => None,
        Some(name) => match extension(name) {
            None => None,
            Some(ext) => Some(ext)
        }
    }
}

fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

fn double_number2(number_str: &str) -> result::Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(err) => Err(err)
    }
}

fn double_number3(number_str: &str) -> result::Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn double_number4(number_str: &str) -> Result<i32> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn double_arg(mut argv: env::Args) -> result::Result<i32, String> {
    argv.nth(1)
        .ok_or("please give at last one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err: ParseIntError| err.to_string()))
        .map(|x| 2 * x)
}

fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let n: i32 = content.trim().parse().unwrap();
    2 * n
}

fn file_double2<P: AsRef<Path>>(file_path: P) -> result::Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|err| err.to_string())
                .map(|_| content)
        })
        .and_then(|content| {
            content.trim().parse::<i32>()
                .map_err(|err: ParseIntError| err.to_string())
        })
        .map(|n| 2 * n)
}

fn file_double3<P: AsRef<Path>>(file_path: P) -> result::Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        return Err(err.to_string());
    }
    let n: i32 = match content.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string())
    };
    Ok(2 * n)
}

fn file_double4<P: AsRef<Path>>(file_path: P) -> result::Result<i32, String> {
    let mut file = try!(File::open(file_path).map_err(|err| err.to_string()));
    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(|e| e.to_string()));
    let n = try!(content.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(2 * n)
}

fn file_double5<P: AsRef<Path>>(file_path: P) -> result::Result<i32, String> {
    let mut file = File::open(file_path).map_err(|err| err.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| e.to_string())?;
    let n = content.trim().parse::<i32>().map_err(|e| e.to_string())?;
    Ok(2 * n)
}

fn file_double6<P: AsRef<Path>>(file_path: P) -> result::Result<i32, CliError> {
    let mut file = File::open(file_path).map_err(CliError::Io)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(CliError::Io)?;
    let n = content.trim().parse::<i32>().map_err(CliError::Parse)?;
    Ok(2 * n)
}

fn file_double7<P: AsRef<Path>>(file_path: P) -> result::Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let n = content.trim().parse::<i32>()?;
    Ok(2 * n)
}

fn file_double8() -> result::Result<f64, CliError> {
    let f = "hel".parse::<f64>()?;
    Ok(f)
}

pub fn demo() {
    common::line();

    let home = "newegg.com";
    match find(home, '.') {
        NewOption::Some(x) => { println!("offset {}", x) }
        NewOption::None => { println!("not found") }
    };
//    find(home, 'x').unwrap(); // error
    println!("extension 1 {}", extension_explict("index.html").unwrap());
    println!("extension 2 {}", extension_explict2("index.html").unwrap());
    println!("extension 3 {}", extension_explict3("index.html").unwrap());
    println!("extension 4 {}", extension_explict3("benjamin").unwrap_or("txt"));
    println!("double number {}", double_number("10"));
//    println!("double number {}", double_number("10z"));

    match double_number2("10z") {
        Ok(n) => assert_eq!(20, n),
        Err(err) => println!("Error {:?}", err),
    }

    match double_number3("100222222222222229292829282282") {
        Ok(n) => assert_eq!(20, n),
        Err(err) => println!("Error {:?}", err),
    }
    match double_number4("10x") {
        Ok(n) => assert_eq!(20, n),
        Err(err) => println!("Error {:?}", err),
    }

    match double_arg(env::args()) {
        Ok(n) => println!("scale from args {}", n),
        Err(err) => println!("{}", err)
    }
    println!("file double from number.dat : {}", file_double("number.dat"));

    match file_double2("number.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{}", err)
    }

    match file_double3("number.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{}", err)
    }
    match file_double4("number.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{}", err)
    }
    match file_double5("number.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{}", err)
    }
    match file_double6("number1.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{:?}", err)
    }

    let io_err: io::Error = io::Error::last_os_error();
    let parse_err: ParseIntError = "not a number".parse::<i32>().unwrap_err();
    let err: ParseFloatError = "num".parse::<f64>().unwrap_err();

    let err1: Box<Error> = From::from(io_err);
    let err2: Box<ParseIntError> = From::from(parse_err);
    let err3: CliError = From::from(err);
    println!("{:?}", err3);

    match file_double7("number1.dat") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{:?}", err)
    }
    match file_double8() {
        Ok(n) => println!("{}", n),
        Err(err) => println!("{:?}", err)
    }
}