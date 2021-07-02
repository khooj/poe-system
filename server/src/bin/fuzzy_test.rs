use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use strsim::{hamming, levenshtein};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matcher = SkimMatcherV2::default();

    println!(
        "{:?}",
        matcher.fuzzy_match("Adds 2 Passive Skills", "Adds 1 Passive Skills")
    );
    println!(
        "{:?}",
        matcher.fuzzy_match("Adds 2 Passive Skills", "Adds 2 Passive Skills")
    );
    println!(
        "{:?}",
        matcher.fuzzy_match(
            "Adds 2 Passive Skills",
            "Added Small Passive Skills also grant: +3% to Chaos Resistance"
        )
    );

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
