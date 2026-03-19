use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    #[allow(unused)]
    Cons(i32, Rc<RcList>),
    Nil,
}

use crate::List::{Cons, Nil};
use crate::RcList::{Cons as RcCons, Nil as RcNil};

#[derive(Debug)]
struct MyBox<T> {
    data: T,
    name: String,
}

impl<T> MyBox<T> {
    fn new(name: String, data: T) -> MyBox<T> {
        MyBox { name, data }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox with name: {}", self.name);
    }
}

#[derive(Debug)]
enum RefRcList {
    Cons(i32, RefCell<Rc<RefRcList>>),
    Nil,
}

impl RefRcList {
    fn tail(&self) -> Option<&RefCell<Rc<RefRcList>>> {
        match self {
            RefRcList::Cons(_, item) => Some(item),
            RefRcList::Nil => None,
        }
    }
}

fn cycle_ref() {
    let a = Rc::new(RefRcList::Cons(5, RefCell::new(Rc::new(RefRcList::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RefRcList::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    match list {
        Cons(data, ref next) => {
            println!("Cons: {}, next: {:?}", data, next);
        }
        Nil => {
            println!("Nil");
        }
    }
    println!("{:?}", list);

    let x = 5;
    let y = MyBox::new("first".into(), x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("start");
    drop(y);
    println!("end");
    // println!("{:?}", y);

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _ = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _ = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    cycle_ref();

    tree();
}
