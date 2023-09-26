mod front_of_house;

pub use front_of_house::serving;
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// #[allow(dead_code)]
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//         fn fix_incorrect_order() {
//             cook();
//             super::serve();
//         }
//         fn cook(){}
//     }

//     fn serve() {}

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//     front_of_house::hosting::add_to_waitlist();
// }
