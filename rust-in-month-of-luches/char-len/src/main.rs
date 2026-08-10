fn main() {
    println!("Size of char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of : {}", " ".len());

    let str1 = "Hello!";
    let str2 = "안녕!";

    println!(
        "str1 is {} bytes, and also number of characters is {}",
        str1.len(),
        str1.chars().count()
    );
    println!(
        "str2 is {} bytes, and also number of characters is {}",
        str2.len(),
        str2.chars().count()
    );

    println!("'a' as bytes: {:?}", "a".as_bytes());
    println!("'ß' as bytes: {:?}", "ß".as_bytes());
    println!("'国' as bytes: {:?}", "国".as_bytes());
}
