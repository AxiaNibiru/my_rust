mod front_of_house;
#[allow(dead_code)]
fn main() {
    front_of_house::serving::complain();
    restaurant::eat_at_restaurant();
}