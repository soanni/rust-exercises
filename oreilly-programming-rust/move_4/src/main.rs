struct Person {
    name: Option<String>,
    birth_date: i32,
}

fn main() {
    let mut composers = vec![Person {
        name: Some("Shestakovich".to_string()),
        birth_date: 1906,
    }];

    // let name = composers[0].name.take();
    // OR the same
    let name = std::mem::replace(&mut composers[0].name, None);

    assert_eq!(name, Some("Shestakovich".to_string()));
}
