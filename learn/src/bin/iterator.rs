use std::{self, vec};

fn print_type_of<T>(_: &T) {
    println!("the type name is: {}", std::any::type_name::<T>())
}

// fn life_test() -> std::slice::Iter<'static, i32> {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();
//     v1_iter
// }

#[test]
fn iterator_demonstration() {
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    print_type_of(&v1_iter.next());
    // v1_iter.sum();

    // assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut into_iter = v1.iter_mut();
    let data = into_iter.next();
    let origin_data = data.unwrap();
    // print_type_of(&data);
    print_type_of(&origin_data);
    *origin_data = 3;
    println!("origin_data is: {origin_data}");
    println!("v1 is: {v1:?}");
    // println!("origin_data is: {origin_data}"); // 放在这会导致v1不能被借用
}

fn main() {
    print_type_of(&String::new());
    let data1 = vec![1, 2, 3];
    let data2 = vec![1, 2, 3];
    println!("data1 == data2 is: {}", data1 == data2);
    let p1 = &data1;
    let p2 = &data2;
    println!("&data1 == &data2 is: {}", std::ptr::eq(p1, p2));

    let doubled = data1.iter().map(|item| item * 2).collect::<Vec<_>>();
    println!("doubled is: {doubled:?}");
}
