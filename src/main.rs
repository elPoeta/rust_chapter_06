#![allow(dead_code, unused)]

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddrS {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum IpAddrE {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrE2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("IPV4: {:?} IPV6: {:?}", IpAddrKind::V4, IpAddrKind::V6);
    }
    {
        let home = IpAddrS {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddrS {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        println!("HOME: {:#?} LOOPBACK: {:#?}", home, loopback);
    }
    {
        let home = IpAddrE::V4(String::from("127.0.0.1"));

        let loopback = IpAddrE::V6(String::from("::1"));
        println!("HOME: {:?} LOOPBACK: {:?}", home, loopback);
    }
    {
        let home = IpAddrE2::V4(127, 0, 0, 1);

        let loopback = IpAddrE2::V6(String::from("::1"));
        println!("HOME: {:?} LOOPBACK: {:?}", home, loopback);
    }
    {
        struct Ipv4Addr {
            address: (u8, u8, u8),
        }

        struct Ipv6Addr {
            address: String,
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        impl Message {
            fn call(&self) {
                // method body would be defined here
                dbg!(self);
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }
    {
        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        let sum = x + y.unwrap();
    }
}
