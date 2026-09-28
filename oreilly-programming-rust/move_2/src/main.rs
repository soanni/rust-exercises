fn main() {
    let mut v = Vec::new();

    for i in 101..106 {
        v.push(i.to_string());
    }

    // let v1 = v[1];
    // let v2 = v[2];
    //
    let v1 = &v[1];
    let v2 = &v[2];

    println!("old v1 and v2 : {v1} and {v2}");

    let fifth = v.pop().expect("vector is empty");
    assert_eq!(fifth, "105");

    let third = v.swap_remove(2);
    assert_eq!(third, "103");

    let second = std::mem::replace(&mut v[1], "substitute".to_string());

    assert_eq!(second, "102");

    println!("{:?}", v);

    assert_eq!(v, ["101", "substitute", "104"]);

    // println!("new v1 and v2: {v1} and {v2}");
    // println!("new v2: {v2}");
}
