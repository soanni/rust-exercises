/// A String has a resizable buffer holding UTF-8 text. The buffer is allocated in the heap, so it can be resized as needed. In the figure, noodles is a String that owns an eight-byte buffer, of which seven are in use. You can think of a String as a Vec<u8> that is guaranteed to hold well-formed UTF-8; in fact, this is how String is implemented.
/// A &str (pronounced “stir” or “string slice”) is a reference to a run of UTF-8 text owned by someone else: it “borrows” the text. In the example, oodles is a &str referring to the last six bytes of the text belonging to noodles, so it represents the text "oodles". Like other slice references, a &str is a fat pointer, containing both the address of the actual data and its length.
/// You can think of a &str as being nothing more than a &[u8] that is guaranteed to hold well-formed UTF-8. Likewise, a char is essentially a u32 that is guaranteed to hold a valid Unicode scalar value. They are types with invariants: rules about their values that are enforced by the implementation, that can’t be broken except by abusing unsafe code, and that programs can therefore rely on.
/// A string literal is a &str that refers to preallocated text, typically stored in read-only memory along with the program’s machine code. In the preceding example, "ಠ_ಠ" is a string literal, pointing to seven bytes that are loaded into memory when the program begins execution and that last until it exits.
/// For now it will suffice to point out that a &str can refer to any slice of any string, so as with slices vs. vectors, &str is usually preferable to String for function arguments.
fn main() {
    let s = "noodles".to_string();
    let ss = &s[1..];
    let sss = &ss[1..];

    let dis = "ಠ_ಠ";
    assert_eq!(dis.len(), 7);
    assert_eq!(dis.chars().count(), 3);

    // immutable
    let m = "mmmm";
    //m[0] = 'k';
    println!("{s} / {ss} / {sss}");

    let mut new_s = String::new();
    new_s.push('h');
    new_s += "e";
    new_s += &("llo".to_string());

    println!("{new_s}");

    let str_v = vec!["bla", "bla", "bla"];

    println!("{}", str_v.join("-"));
    println!("{}", str_v.concat());
}
