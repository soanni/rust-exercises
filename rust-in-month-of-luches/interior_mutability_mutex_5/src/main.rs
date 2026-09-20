use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(39);
    let mut mutex_guard = my_mutex.lock().unwrap();
    let mut mutex_guard_1 = my_mutex.try_lock();

    if let Ok(value) = mutex_guard_1 {
        println!("the value inside mutex is {}", value);
    } else {
        println!("didn't get a lock");
    }
}
