enum Stars {
    Dwarf = 10,
    RedDwarf = 100,
    Giant = 200,
    RedGiant = 1000,
    SuperGiant,
}

fn main() {
    use Stars::*;

    let arr = vec![Dwarf, RedDwarf, Giant, RedGiant, SuperGiant];

    for s in arr {
        match s as u32 {
            size if size <= 80 => println!("the star is not too big, its size is {size}"),
            size if size > 80 && size < 200 => {
                println!("the star is pretty big its size is {size}")
            }
            other => println!("the star is big and the size is {other}"),
        }
    }
}
