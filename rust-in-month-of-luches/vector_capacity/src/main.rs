fn main() {
    let mut v = Vec::new();
    println!("len: {} , cap: {}", v.len(), v.capacity());
    v.push('a');
    println!("len: {} , cap: {}", v.len(), v.capacity());
    v.push('a');
    v.push('a');
    v.push('a');
    println!("len: {} , cap: {}", v.len(), v.capacity());
    v.push('a');
    println!("len: {} , cap: {}", v.len(), v.capacity());
}
