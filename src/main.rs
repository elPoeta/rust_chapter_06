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
    {
        #[derive(Debug)] // so we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("six {}", six.unwrap());
    }
    {
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn reroll() {}
    }
}
