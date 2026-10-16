fn main() {
    let dim = (30, 50);

    println!("the area is {}", area(dim));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
