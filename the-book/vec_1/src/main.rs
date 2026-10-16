#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut vv = vec![1, 2, 3, 4, 5];
    let mut vvv = Vec::new();

    vvv.push(1);
    vvv.push(2);
    vvv.push(3);
    vvv.push(4);
    vvv.push(5);

    let third = &vv[2];
    println!("The third element is {third}");

    let fourth: Option<&i32> = vv.get(3);
    match fourth {
        Some(num) => println!("The fourth element is {num}"),
        None => println!("There is no fourth element at all"),
    }

    //let does_not_exist = &vv[100];
    //let does_not_exist = vv.get(100);
    // vv.push(6);
    println!("The third element is {third}");

    // iterate over immutable references
    for i in &vv {
        println!("{i}");
    }

    // iterate over mutable references of a mutable vector

    for i in &mut vvv {
        *i += 50;
    }

    println!("{:?}", vvv);

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("text")),
        SpreadsheetCell::Float(3.14),
    ];

    println!("{row:?}");
}
