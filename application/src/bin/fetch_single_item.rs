use application::*;

#[tokio::main]
async fn main() {
    let pob = include_str!("pob.txt");
    calculate_build_cost(pob)
        .await
        .expect("can't calculate build cost");
}
