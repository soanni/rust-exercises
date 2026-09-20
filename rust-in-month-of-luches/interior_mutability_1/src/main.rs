use std::cell::Cell;

#[derive(Debug)]
struct PhoneModel {
    company: String,
    model: String,
    screen_size: f32,
    memory: usize,
    on_sale: Cell<bool>,
}

impl PhoneModel {
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

fn main() {
    let iphone_7 = PhoneModel {
        company: "Apple".to_string(),
        model: "IPhone 7".to_string(),
        screen_size: 8.5,
        memory: 32_000_000,
        on_sale: Cell::new(true),
    };

    iphone_7.make_not_on_sale();
    println!("{:#?}", iphone_7);
}
