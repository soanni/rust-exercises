use std::fmt;

#[derive(Debug)]
struct Dog {
    name: String,
}

struct Parrot {
    name: String,
    age: u8,
}

impl fmt::Display for Parrot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - is a parrot, {} years old", self.name, self.age)
    }
}

fn print_parrot(input: String) {
    println!("{input} - SUPER PARROT !!!");
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
        age: 2,
    };

    spike.bark();
    spike.run();
    gosha.bark();
    gosha.run();

    println!("{spike:?}");
    println!("{gosha}");

    print_parrot(gosha.to_string());
    println!(
        "Super parrot gosha String has {} characters",
        gosha.to_string().chars().count()
    );
}
