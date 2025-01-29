use std::env::args;

use utils::stream_stashes::open_stashes;
use public_stash::models::PublicStashData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().into_iter().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <stashses archive or dir>", args[0]);
        return Ok(());
    }

    let stashes = open_stashes(&args[1]);

    let mut count = 0usize;
    let mut size = 0usize;
    let mut stashes_in_file = 0usize;
    let mut zero_stashes = 0usize;
    let mut items_count = 0usize;
    let max = stashes
        .filter_map(|s| serde_json::from_str::<PublicStashData>(&s.1).ok())
        .map(|s| s.stashes)
        .flat_map(|s| {
            stashes_in_file = stashes_in_file.max(s.len());
            zero_stashes = zero_stashes.max(
                s.iter()
                    .fold(0, |acc, s| acc + if s.items.is_empty() { 1 } else { 0 }),
            );
            s
        })
        .map(|s| {
            count += s.items.len();
            items_count += s.items.len();
            size += s
                .items
                .iter()
                .map(|i| std::mem::size_of_val(i))
                .sum::<usize>();
            s.items.len()
        })
        .max_by(|a, b| a.cmp(&b));
    println!("max items in single stash: {}", max.unwrap());
    println!("avg deserialized item size is: {} bytes", size / count);
    println!("max stashes in one file: {}", stashes_in_file);
    println!("maximum empty stashes found: {}", zero_stashes);
    println!("items sum (without deletion): {}", items_count);
    Ok(())
}
