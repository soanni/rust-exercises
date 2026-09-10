#[derive(Debug)]
struct Library {
    name: String,
    books: BooksCollection,
}

#[derive(Debug, Clone)]
struct BooksCollection(Vec<String>);

impl Library {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BooksCollection(Vec::new()),
        }
    }

    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn get_books(&self) -> BooksCollection {
        self.books.clone()
    }
}

impl Iterator for BooksCollection {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            Some(book) => {
                println!("Book {book} is accessed");
                Some(book)
            }
            None => {
                println!("No more books in the library");
                None
            }
        }
    }
}

fn main() {
    let mut lib = Library::new("my lib");
    //    println!("{lib:?}");
    lib.add_book("Book 1");
    lib.add_book("Book 2");
    lib.add_book("Book 3");
    lib.add_book("Book 4");

    for book in lib.get_books() {
        println!("{book}");
    }

    for book in lib.get_books() {
        println!("{book}");
    }
}
