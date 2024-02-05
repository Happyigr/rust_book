// V4 is a function that returns a String
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct QuitMessage;

enum Message {
    QuitMessage,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
