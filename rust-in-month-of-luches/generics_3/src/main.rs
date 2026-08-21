use std::fmt::Display;

fn announce_and_compare<T, U>(statement: T, input1: U, input2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{statement}. Is {input1} > {input2} ? {}", input1 > input2);
}

fn main() {
    announce_and_compare("Listen Up!", 8, 7);
}
