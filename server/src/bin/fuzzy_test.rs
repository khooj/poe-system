use strsim::{hamming, levenshtein};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{:?}",
        hamming("Adds 2 Passive Skills", "Adds 1 Passive Skills")
    );
    println!(
        "{:?}",
        hamming("Adds 22 Passive Skills", "Adds 2 Passive Skills")
    );
    println!(
        "{:?}",
        hamming(
            "Adds 2 Passive Skills",
            "Added Small Passive Skills also grant: +3% to Chaos Resistance"
        )
    );

    println!(
        "{:?}",
        levenshtein("Adds 2 Passive Skills", "Adds 1 Passive Skills")
    );
    println!(
        "{:?}",
        levenshtein("Adds 2 Passive Skills", "Adds 2 Passive Skills")
    );
    println!(
        "{:?}",
        levenshtein(
            "Adds 2 Passive Skills",
            "Added Small Passive Skills also grant: +3% to Chaos Resistance"
        )
    );

    Ok(())
}
