fn main() {
    let i = vec!["even", "odd"].iter().cycle();
    let v = (0..=5).zip(i).collect::<Vec<(i32, &&str)>>();
    println!("{:?}", v);
}
