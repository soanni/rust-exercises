enum IpAddrKind {
    V4,
    V6,
}

fn route(kind: IpAddrKind) {
    println!("... routed ...")
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
