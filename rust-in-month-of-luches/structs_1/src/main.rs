struct UnitExample;

#[derive(Debug)]
struct ColorRgb(u8, u8, u8);

struct SizeAndColour {
    size: u32,
    colour: ColorRgb,
}

fn main() {
    let colour = ColorRgb(50, 0, 25);
    let example = SizeAndColour {
        size: 132,
        colour: colour,
    };
    println!("second part of colour is {}", example.colour.1);
    //println!("{:?}", colour);
}
