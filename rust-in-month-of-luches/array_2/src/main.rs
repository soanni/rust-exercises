fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice1 = &arr[..];
    let slice2 = &arr[1..5];
    let slice3 = &arr[1..=5];
    let slice4 = &arr[1..];
    let slice5 = &arr[..5];
    println!("{:?}", slice1);
    println!("{:?}", slice2);
    println!("{:?}", slice3);
    println!("{:?}", slice4);
    println!("{:?}", slice5);
    slice5.abcd();
}
