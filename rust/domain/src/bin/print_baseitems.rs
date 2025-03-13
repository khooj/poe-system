use std::ops::Deref;

use domain::data::BASE_ITEMS;

fn main() {
    println!("{:?}", BASE_ITEMS.deref());
}
