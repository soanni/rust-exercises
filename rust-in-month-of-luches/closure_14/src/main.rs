fn main() {
    let arr = ["8", "9", "Hi", "9898989898"];
    let mut v = vec![];

    for i in 0..5 {
        v.push(
            arr.get(i)
                .and_then(|n| n.parse::<u32>().ok())
                .and_then(|n| char::try_from(n).ok()),
        );
    }

    println!("{v:?}");
}
