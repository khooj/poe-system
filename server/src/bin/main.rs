use poe_system::application::{configuration::get_configuration, startup::Application};
use tokio::signal;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let application = Application::build(configuration).await?;
    signal::ctrl_c().await?;
    application.stop().await?;

    Ok(())
}
