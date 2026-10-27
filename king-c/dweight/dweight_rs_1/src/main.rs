use std::{
    error::Error,
    io::{self, Write},
    process::exit,
};

fn prompt(msg: &str) -> Result<i32, Box<dyn Error>> {
    print!("{msg}: ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let param = buf.trim().parse::<i32>()?;
    //let param = buf.trim().parse::<i32>().expect("not a number");
    Ok(param)
}

fn run() -> Result<(), Box<dyn Error>> {
    let length = prompt("Enter the length of the box")?;
    let width = prompt("Enter the width of the box")?;
    let height = prompt("Enter the height of the box")?;
    let volume = length * width * height;
    let weight = (volume + 165) / 166;
    print!("Dims: {length} x {width} x {height}\n");
    print!("Volume: {volume}\n");
    print!("Weight: {weight}\n");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        exit(1);
    }
}
