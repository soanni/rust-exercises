fn main() {
    //let length: i32 = 12;
    //let width: i32 = 8;
    //let height: i32 = 10;
    //let volume;
    //let weight;
    let (length, width, height) = (12, 8, 10);
    let volume = length * width * height;
    let weight = (volume + 165) / 166;
    print!("Dims: {length} x {width} x {height}\n");
    print!("Volume: {volume}\n");
    print!("Weight: {weight}\n");
}
