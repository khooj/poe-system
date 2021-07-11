use molt::Interp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tcl = Interp::new();

    molt_shell::script(&mut tcl, &["./src/bin/script.tcl".to_owned()]);

    Ok(())
}
