enum IpAddr {
    V4(String),
    V6(String),
}

fn emum_learn() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    match home {
        IpAddr::V4(ip) => println!("home ip: {ip}"),
        IpAddr::V6(ip) => println!("loopback ip: {ip}"),
    }
}

fn test_if_let() {
    let mut config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    config_max = Some(10);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn main() {
    println!("Hello, world!");
    slice();
    emum_learn();
    test_if_let();

    let mut ret = describe_state_quarter(Coin::Quarter(UsState::Alabama)).unwrap();
    println!("{ret}");
    ret = describe_state_quarter(Coin::Quarter(UsState::Alaska)).unwrap();
    println!("{ret}");
    match describe_state_quarter(Coin::Penny) {
        Some(ret) => println!("{ret}"),
        None => println!("{}", "None"),
    }
}
