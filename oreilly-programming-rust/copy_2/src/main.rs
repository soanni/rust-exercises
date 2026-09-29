#[derive(Copy, Clone)]
struct Label {
    number: i32,
}

fn print_label(l: Label) {
    println!("{}", l.number);
}

fn main() {
    let l = Label { number: 39 };

    print_label(l);

    println!("{}", l.number);
}
