fn main() {
    print!("\tHello, world!\n\tBeautiful world!\n");
    println!(r#"He said that you can find a file at c:\files\file.txt"#);
    println!("{:?}", b"This will look like numbers");
    println!("{:?}", br##"I like to write "#""##);
    let num = 8;
    let ref_num = &num;
    dbg!(ref_num);
    println!("{:p}", ref_num);
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", num, num, num);
}
