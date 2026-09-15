fn main() {
    let v = vec!["8.9", "eight point nine", "11.2", "seven-eleven", "9.9"];

    let valid_floats = v
        .iter()
        .filter_map(|f| f.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    let valid_floats_1 = v
        .iter()
        .filter_map(|f| f.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    println!("{valid_floats:?}");
    println!("{valid_floats_1:?}");
}
