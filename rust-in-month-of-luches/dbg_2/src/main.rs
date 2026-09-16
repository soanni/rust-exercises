fn main() {
    let mut n = dbg!(1);
    dbg!(n += 1);
    let v = dbg!(vec![1, 2, 3]);
    let vv = dbg!(v.into_iter().map(|x| x * 2).collect::<Vec<i32>>());
}
