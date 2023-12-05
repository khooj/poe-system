use crate::configuration::Settings;
use crate::{
    application::build_calculating::BuildCalculating,
    infrastructure::repositories::create_repositories,
};
use public_stash::client::Client;

use crate::infrastructure::controller::{get_build, new_build};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing::error;
use tracing_actix_web::TracingLogger;

const USER_AGENT: &str = "OAuth poe-system/0.0.1 (contact: bladoff@gmail.com)";

pub struct Application {
    listener: TcpListener,
    interval: u64,
    enable: bool,
    build_calculating: BuildCalculating,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        Application::setup_tracing();

        let (tasks, builds) = create_repositories(&configuration.database)
            .await
            .expect("can't create repositories");

        let client = Client::new(USER_AGENT.to_owned());

        if configuration.start_change_id.is_some() {}

        let build_calculating = BuildCalculating::new(tasks, builds);

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let enable_items_refresh = configuration.application.enable_items_refresh;
        let refresh_interval_secs = configuration.application.refresh_interval_secs;

        let listener = TcpListener::bind(&addr)?;

        Ok(Self {
            listener,
            interval: refresh_interval_secs,
            enable: enable_items_refresh,
            build_calculating,
        })
    }

    fn setup_tracing() {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};

        let fmt_subscriber = fmt::layer();

        let env_subscriber = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("poe-system")
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("can't init opentelementry");

        let opentelemetry_jaeger = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(fmt_subscriber)
            .with(env_subscriber)
            .with(opentelemetry_jaeger)
            .try_init()
            .expect("can't init tracing subscribers");
    }

    // TODO: graceful shutdown?
    pub async fn run(self) -> Result<(), std::io::Error> {
        use tokio::time::{interval, Duration};

        let Application {
            listener: l,
            build_calculating: bc,
            ..
        } = self;

        let bc1 = bc.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("cant create runtime");
            rt.block_on(async move {
                let mut interval = interval(Duration::from_millis(100));
                loop {
                    interval.tick().await;
                    let ret = bc1.calculate_next_build().await;
                    if let Err(e) = ret {
                        error!("error calculating next build: {}", e);
                    }
                }
            });
        });

        HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(bc.clone()))
                .service(new_build)
                .service(get_build)
        })
        .workers(4)
        .listen(l)
        .expect("cant listen actix-web server")
        .run()
        .await
        .expect("cant run actix-web server");
        Ok(())
    }
}
