use trait_obj_2::{Draw, Screen, SelectBox, TextField};

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button.draw() is called");
    }
}

impl Button {
    fn click(&self) {
        println!("Button.click() is called");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            TextField {
                width: 10,
                height: 11,
                label: String::from("label1"),
            },
            //            Box::new(SelectBox {
            //                width: 5,
            //                height: 4,
            //                options: vec![
            //                    String::from("opt1"),
            //                    String::from("opt2"),
            //                    String::from("opt3"),
            //                ],
            //            }),
            //            Box::new(Button {
            //                width: 9,
            //                height: 12,
            //                label: String::from("super btn"),
            //            }),
        ],
    };

    screen.run();
}
