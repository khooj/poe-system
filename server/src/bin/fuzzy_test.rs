use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

fn main() -> Result<(), std::io::Error> {
    let matcher = SkimMatcherV2::default();

    println!("{:?}", matcher.fuzzy_match("Adds 2 Passive Skills", "Adds 1 Passive Skills"));
    println!("{:?}", matcher.fuzzy_match("Adds 2 Passive Skills", "Adds 2 Passive Skills"));
    println!("{:?}", matcher.fuzzy_match("Adds 2 Passive Skills", "Added Small Passive Skills also grant: +3% to Chaos Resistance"));
    Ok(())
}