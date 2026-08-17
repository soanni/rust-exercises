fn main() {
    let mut v = Vec::with_capacity(8);
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
