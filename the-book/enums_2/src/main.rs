enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(kind: IpAddrKind) {
    println!("... routed ...")
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);
}
