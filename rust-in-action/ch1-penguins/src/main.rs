fn main() {
    let text = "\
Breed, length(cm)
Breed 1, 100.0,
Breed 2, 98.9
Breed 3, 101
Breed 4, 121.1
No data, No data";
    for (i, record) in text.lines().enumerate() {
        let parts: Vec<_> = record.split(',').map(|f| f.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("Record: {:?}, Parts: {:?}", record, parts);
        }

        if let Ok(l) = parts[1].parse::<f32>() {
            println!("{} - {}", parts[0], l);
        }
    }
}
