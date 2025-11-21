fn test(s: String) {
    println!("{}", s);
}

fn main() {
    let mut s = String::from("hello");
    test(s); // s已经被移动了
    s = "world".to_string();
    println!("{}", s);
}
