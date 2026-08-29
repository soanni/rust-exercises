use std::collections::HashMap;

fn main() {
    let books = vec!["Book1", "Book2", "Book3", "Book4", "Book1"];

    let mut books_hash_map = HashMap::new();

    for book in books {
        books_hash_map.entry(book).or_insert(true);
    }

    for (k, v) in books_hash_map {
        println!("Do we have a book {k} ? {v}");
    }
}
