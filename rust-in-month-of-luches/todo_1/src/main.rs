#[derive(Debug)]
struct Book {
    title: String,
    year: u8,
}

enum BookType {
    HardCover,
    SoftCover,
}

fn get_book(book: &Book) -> Option<String> {
    todo!();
}

fn delete_book(book: &Book) -> Result<(), String> {
    todo!();
}

fn check_type(book_type: &BookType) {
    match book_type {
        BookType::HardCover => println!("it's a harcover"),
        BookType::SoftCover => println!("it's a softcover"),
    }
}

fn make_book() -> Book {
    Book {
        title: todo!(),
        year: todo!(),
    }
}

fn main() {
    check_type(&BookType::HardCover);
    //    let b = make_book();
    //    println!("{b:?}");
}
