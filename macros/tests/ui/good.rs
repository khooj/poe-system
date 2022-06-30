use macros::static_array_from_file;

static_array_from_file!(TYPES, "../../../macros/tests/ui/types.txt");

fn main() {
    TYPES.len();
}