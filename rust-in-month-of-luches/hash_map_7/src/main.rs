use std::collections::HashMap;

fn main() {
    let books = vec!["Book1", "Book2", "Book3", "Book4", "Book1"];

    let mut books_hash_map = HashMap::new();

    for book in books {
        let val = books_hash_map.entry(book).or_insert(0);
        *val += 1;
    }

    for (k, v) in books_hash_map {
        println!("How many copies of the book do we have ? book {k} - {v}");
    }
}
