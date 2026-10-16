fn main() {
    let s1 = "hi".to_string();
    // let h = s1[0];
    let hello = "Здравствуйте";
    // indexing doesnt work
    // let answer = &hello[0];
    // BUT string slicing DOES work
    // the below one takes 4 bytes or зд
    let h = &hello[0..4];
    println!("{h}");
    // but 0..1 or 0..3 will not work and cause crash in runtime
    // let h2 = &hello[0..3];
    // println!("{h2}");
    for c in h.bytes() {
        println!("{c}");
    }

    for c in h.chars() {
        println!("{c}");
    }
}
