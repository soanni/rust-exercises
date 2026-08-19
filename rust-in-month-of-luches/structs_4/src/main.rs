enum AnimalType {
    Cat,
    Dog,
}

struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("It's a cat"),
            AnimalType::Dog => println!("It's a dog"),
        }
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat");
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog");
    }
}

fn main() {
    let mut a = Animal::new_cat();
    a.check_type();
    a.change_to_dog();
    a.check_type();
    a.change_to_cat();
    a.check_type();
}
