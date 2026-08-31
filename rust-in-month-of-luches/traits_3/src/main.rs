struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self);

    fn run(&self);
}

impl DogLike for Dog {
    fn bark(&self) {
        println!("{}, the dog stop barking!", self.name);
    }

    fn run(&self) {
        println!("{} the dog is running", self.name);
    }
}
impl DogLike for Parrot {
    fn bark(&self) {
        println!("{}, the parrot stop barking!", self.name);
    }

    fn run(&self) {
        println!("{} the parrot is running", self.name);
    }
}

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
    gosha.run();
}
