fn main() {
    let float_1 = 5.0f32;
    let float_2: f64 = 5.1;
    // mismatched types
    // let sum = float_1 + float_2;
    let sum = float_1 + float_2 as f32;
    println!("{}", sum);
    let my_f_1: f32 = 0.1;
    let m_f_2 = 0.2;
    let my_sum = my_f_1 + m_f_2;
    println!("{}", my_sum);
    {
        let m_f_3 = 0.3;
    }

    //    let my_sym_2 = my_sum + m_f_3;
    println!(
        "the min for char is {}, the max is {}",
        i128::MIN,
        i128::MAX
    );
}
