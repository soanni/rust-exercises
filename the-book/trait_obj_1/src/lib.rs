///  A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime. We create a trait object by specifying some sort of pointer, such as a reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait. We can use trait objects in place of a generic or concrete type. Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will implement the trait object’s trait. Consequently, we don’t need to know all the possible types at compile time.
/// Trait objects differ from objects in other languages in that we can’t add data to a trait object. Trait objects aren’t as generally useful as objects in other languages: Their specific purpose is to allow abstraction across common behavior
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
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
