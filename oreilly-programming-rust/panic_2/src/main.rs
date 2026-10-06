struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping noisy {}", self.0);
    }
}

fn inner() {
    let _inner = Noisy("inner");
    panic!("boom");
}

fn main() {
    let outer = Noisy("outer");
    inner();
}
