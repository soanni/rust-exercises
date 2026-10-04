// fix for ref_8
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
}
