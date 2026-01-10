fn test(s: String) {
    println!("{}", s);
}

fn main() {
    let mut s = String::from("hello");
    test(s); // s已经被移动了
    s = "world".to_string();
    println!("{}", s);
    let s1 = String::from("hello");
    // let s2 = s1; // s1被移动了
    // s1 = "world".to_string();  // s1不可变 不能赋值
    // println!("{}, {}", s2, s1);
    println!("{}", s1);
}
