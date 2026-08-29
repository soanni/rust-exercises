fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut v in weather_vec {
        println!("For the city {}", v[0]);
        while let Some(info) = v.pop() {
            if let Ok(n) = info.parse::<i32>() {
                println!("{n}");
            }
        }
    }
}
