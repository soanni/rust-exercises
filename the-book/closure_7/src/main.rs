#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut v = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 3,
        },
        Rectangle {
            width: 7,
            height: 5,
        },
    ];

    v.sort_by_key(|r| r.width);

    println!("{v:#?}");
}
