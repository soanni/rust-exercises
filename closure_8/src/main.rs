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
    let mut l = vec![];
    // closure becomes FnOnce as it moves the value out of the env in this case and CAN'T be called
    // more than once
    //let value = String::from("closure is called");
    let value = "closure is called";
    v.sort_by_key(|r| {
        l.push(value);
        r.width
    });

    println!("{v:#?}");
    println!("{l:#?}");
}
