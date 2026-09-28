struct Label {
    number: i32,
}

fn print_label(l: Label) {
    println!("{}", l.number);
}

fn main() {
    let l = Label { number: 39 };

    print_label(l);

    // wont work as Label is not a copy type however i32 is
    // println!("{}", l.number);
}
