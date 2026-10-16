struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("cleaning CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    let p1 = CustomSmartPointer {
        data: String::from("some stuff"),
    };

    let p2 = CustomSmartPointer {
        data: String::from("some other stuff"),
    };

    drop(p2);
    println!("Dropped p2 before the end of main");
}
