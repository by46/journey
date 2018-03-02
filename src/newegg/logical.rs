use example::common;

#[derive(Debug, PartialEq)]
enum BookFormat { Paperback, Hardback, Ebook }

#[derive(Debug)]
struct Book {
    isbn: i32,
    format: BookFormat,
}

#[derive(Debug)]
enum Day {
    Sunday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Monday,
}
//impl PartialEq for BookFormat {
//    fn eq(&self, other: &BookFormat) -> bool {
//        let result = match self {
//            other => true,
//            _ => false
//        };
//        println!("BookFormat debugging {:?} == {:?}? {}", self, other, result);
//        result
//    }
//}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        println!("debugging PartialEq");
        self.isbn == other.isbn //&& self.format == other.format
    }
}

impl Eq for Book {}

pub fn demo() {
    common::line();
    let book1 = Book { isbn: 1, format: BookFormat::Ebook };
    let book2 = Book { isbn: 1, format: BookFormat::Hardback };
    //    println!("{:?} == {:?}? {}", book1, book2, book1 == book2);
    //    println!("{:?} == {:?}? {}", BookFormat::Hardback, BookFormat::Paperback, BookFormat::Hardback == BookFormat::Paperback);

    let paper = BookFormat::Paperback;
    let hard = BookFormat::Hardback;
    match paper {
        hard => println!("hard"),
        BookFormat::Ebook => println!("Ebook"),
        BookFormat::Hardback => println!("Hardback"),
        BookFormat::Paperback => println!("Paperback")
    }

}