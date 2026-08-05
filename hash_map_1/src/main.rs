use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    hm.insert(String::from("blue"), 10);
    hm.insert(String::from("yellow"), 50);
    hm.insert(String::from("black"), 90);
    hm.insert(String::from("white"), 100);

    let team_name = String::from("blue");
    let score = hm.get(&team_name).copied().unwrap_or(0);
    println!("team={team_name}, score={score}");

    for (k, v) in &hm {
        println!("team={k}, score={v}");
    }

    let mut hm_1 = HashMap::new();

    let field_name = "Favorite color".to_string();
    let field_value = "orange".to_string();

    hm_1.insert(&field_name, &field_value);

    println!("{hm_1:?}");
    println!("{field_name}");
    println!("{field_value}");

    let field_value_1 = "red".to_string();

    hm_1.insert(&field_name, &field_value_1);
    println!("{hm_1:?}");

    let ref1 = *hm.entry(String::from("red")).or_insert(999);
    let ref2 = *hm.entry(String::from("blue")).or_insert(9999);

    println!("{hm:?}");
    println!("{ref1:?}");
    println!("{ref2:?}");
}
