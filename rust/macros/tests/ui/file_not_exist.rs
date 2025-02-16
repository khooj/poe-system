use macros::static_array_from_file;

pub static TYPES: &[&'static str] = static_array_from_file!("file.txt");

fn main() {}