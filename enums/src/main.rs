enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Ipv4Addr(u8, u8, u8, u8);

struct Ipv6Addr(String);

struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(Ipv4Addr(127, 0, 0, 1));

    let loopback = IpAddr::V6(Ipv6Addr(String::from("::1")));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}

