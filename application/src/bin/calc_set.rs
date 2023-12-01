use application::CalculatingState;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    pob: Option<String>,
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    let mut state = CalculatingState::default();
    state.parse_pob(cli.pob.unwrap())?;
    state
        .calculate_build_cost(cli.output.as_ref().unwrap())
        .await?;
    Ok(())
}
