fn main() {
    //let a: [_; 0] = [];
    let mut a = [true; 10000];

    for i in 2..100 {
        if a[i] {
            let mut j = i * i;
            while j < 10000 {
                a[j] = false;
                j += i;
            }
        }
    }

    //    println!("{:#?}", a);
    //
    //let n = 3;

    //let a_1 = [0u8; n];
    //
    // 1 Kb buffer filler with 0s
    //
    let b = [0u8; 1024];
}
