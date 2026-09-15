fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|x| x.len() < 5)
        .filter(|x| x.contains('u'))
        .collect();
    println!("{filtered_months:?}");
}
