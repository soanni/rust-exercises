use std::process::Command;

fn main() {
    let out = Command::new("./connection_bin")
        .output()
        .expect("Failed to start connection");

    if out.status.success() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        println!("output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        eprintln!("Error: {}", stderr);
    }
}
