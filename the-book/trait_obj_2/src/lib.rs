/// This works differently from defining a struct that uses a generic type parameter with trait bounds. A generic type parameter can be substituted with only one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        self.components.iter().for_each(|c| c.draw());
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("TextField.draw() is called");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox.draw()");
    }
}
