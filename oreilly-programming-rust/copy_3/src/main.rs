#[derive(Copy, Clone)]
struct StringLabel {
    name: String,
}

fn print_label(l: StringLabel) {
    println!("{}", l.name);
}

fn main() {
    let l = StringLabel {
        name: "Bill".to_string(),
    };

    print_label(l);

    println!("{}", l.name);
}
