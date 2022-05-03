use poe_system::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run().await?;

    Ok(())
}
