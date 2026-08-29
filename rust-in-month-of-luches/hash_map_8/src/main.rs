use std::collections::HashMap;

fn main() {
    let data = [
        ("male", 3),
        ("female", 4),
        ("male", 2),
        ("female", 9),
        ("female", 10),
        ("male", 7),
        ("male", 2),
    ];

    let mut political_survey = HashMap::new();

    for (k, v) in data {
        political_survey.entry(k).or_insert(Vec::new()).push(v);
    }

    for (k, v) in political_survey {
        println!("{k} - {v:?}");
    }
}
