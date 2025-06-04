mod front_of_house {
    pub mod hosting {
        use crate::front_of_house::serving;
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
            seat_at_table();
            serving::serve_order();
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {
            take_order();
            take_payment();
        }

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
