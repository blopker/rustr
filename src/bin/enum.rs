use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

enum IpAddrKind {
    V4,
    V6,
}

struct MyIpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}

impl Message {
    fn call(&self) {
        println!("called");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = MyIpAddr {
        kind: IpAddrKind::V4,
        address: String::from("12.12.12.12"),
    };

    let office = MyIpAddr {
        kind: IpAddrKind::V6,
        address: String::from("12:12:12:12"),
    };

    let home2 = IpAddr2::V4(String::from("12.12.12.12"));
    let office = IpAddr2::V6(String::from("::1"));

    let home = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
    let office = IpAddr::V6(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8));

    let msg = Message::Quit;
    msg.call();
}

fn route(ip_type: IpAddrKind) {}
