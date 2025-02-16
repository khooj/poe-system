use std::ops::Deref;

use domain::data::BASE_TYPES;

fn main() {
    println!("{:?}", BASE_TYPES.deref());
}
