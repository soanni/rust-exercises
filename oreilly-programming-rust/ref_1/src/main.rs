use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut t = Table::new();
    t.insert(
        "artist 1".to_string(),
        vec![
            "work 11".to_string(),
            "work12".to_string(),
            "work13".to_string(),
            "work14".to_string(),
        ],
    );
    t.insert(
        "artist 2".to_string(),
        vec![
            "work 21".to_string(),
            "work22".to_string(),
            "work23".to_string(),
        ],
    );
    t.insert(
        "artist 3".to_string(),
        vec!["work 31".to_string(), "work32".to_string()],
    );

    sort_works(&mut t);

    show(&t);
}

fn show(tab: &Table) {
    // every artist now is a &String
    for (artist, works) in tab {
        println!("Artist: {artist}");
        // works is now &Vec<String> and every work is a &String
        for w in works {
            println!("\t {w}");
        }
    }
}

fn sort_works(tab: &mut Table) {
    for (_, works) in tab {
        works.sort();
    }
}
