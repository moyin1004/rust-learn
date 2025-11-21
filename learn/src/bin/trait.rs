fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> std::fmt::Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.part)
    }
}

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    let latest = learn::generic::lastest(&mut list);
    println!("{latest}");
    *latest = 20;
    println!("{list:?}");

    let p = learn::generic::Point::new(1, 2);
    println!("{}", p);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i);
}
