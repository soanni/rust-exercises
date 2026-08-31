struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof-Woof");
    }

    fn run(&self) {
        println!("The dog is running");
    }
}

impl DogLike for Dog {}
impl DogLike for Parrot {}

fn main() {
    let spike = Dog {
        name: "Spike".to_string(),
    };

    let gosha = Parrot {
        name: "Gosha".to_string(),
    };

    spike.bark();
    spike.run();
    gosha.bark();
}
