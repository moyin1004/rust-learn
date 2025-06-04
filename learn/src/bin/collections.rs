fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    println!("{v:?}");
    let mut v = vec![1, 2, 3];
    v.push(4);
    let third = &mut v[2];
    *third += 5;
    println!("The third element is {}", v[2]);

    let strings = vec![String::from("hello"), String::from("world")];
    let mut s = &strings[0]; // 借用
    println!("{s}");
    let new_s = String::from("hello world");
    s = &new_s; // 修改s
    println!("{}, {strings:?}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}, {s1}");

    let zhong = String::from("中午耐高温");
    for c in zhong.chars() {
        print!("{c}");
    }
    println!();
    for byte in zhong.bytes() {
        print!("{byte}");
    }
    println!();

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(String::clone(&field_name), field_value);
    println!("{field_name}");

    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(10); // 不存在时才插入
    println!("{:?}", scores);
}
