use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(39);
    {
        let mut mutex_guard = my_mutex.lock().unwrap();
        println!("{my_mutex:?}");
        println!("{mutex_guard:?}");
        *mutex_guard = 6;
    }
    println!("{my_mutex:?}");
}
