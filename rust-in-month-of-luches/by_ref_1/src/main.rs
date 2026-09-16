fn main() {
    let mut i = [1, 2, 3, 4].into_iter();
    let ar1 = i.by_ref().take(2).collect::<Vec<_>>();
    let ar2 = i.take(2).collect::<Vec<_>>();
    println!("{ar1:?}");
    println!("{ar2:?}");
}
