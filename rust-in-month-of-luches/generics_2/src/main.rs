use std::fmt::Display;

fn announce_and_compare<T: Display, U: Display + PartialOrd>(statement: T, input1: U, input2: U) {
    println!("{statement}. Is {input1} > {input2} ? {}", input1 > input2);
}

fn main() {
    announce_and_compare("Listen Up!", 8, 7);
}
