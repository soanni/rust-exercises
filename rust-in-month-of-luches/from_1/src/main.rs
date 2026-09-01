fn main() {
    let v1 = Vec::from([1, 2, 3, 4]);
    println!("Vec from array: {v1:?}");

    let v2 = Vec::from("a string a string");
    println!("Vec from str: {v2:?}");

    let v3 = Vec::from("a string a string".to_string());
    println!("Vec from String: {v3:?}");
}
