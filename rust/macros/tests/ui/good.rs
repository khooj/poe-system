use macros::static_array_from_file;

pub static TYPES: &[&'static str] =
    static_array_from_file!("../../../../macros/tests/ui/types.txt");

fn main() {
    let _ = TYPES.len();
}

