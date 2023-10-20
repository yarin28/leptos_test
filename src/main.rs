#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos_start::utils::config_builder;
    leptos_start::utils::config_builder::config_build();
    println!("finished");
    Ok(())
}
async fn main2() -> std::io::Result<()> {
    use actix::prelude::*;
    use actix_files::Files;
    use actix_web::middleware::Logger;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_start::api::check_health::check_health;
    use leptos_start::utils::configure_logger;
    use std::process;
    // use leptos_start::app::ChangeCronString;
    use leptos_start::app::*;
    use leptos_start::my_scheduler::*;
    use leptos_start::utils::LowLevelHandler;
    use tracing::event;
    use tracing::Level;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;
    // configure_logger::configure_logger();

    //configure the logger
    let file_appender = tracing_appender::rolling::daily("./logs", "log_of_day");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_span_events(
                    tracing_subscriber::fmt::format::FmtSpan::CLOSE
                        | tracing_subscriber::fmt::format::FmtSpan::ENTER,
                ),
        )
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("none,leptos_start=trace"))
                .unwrap(),
        )
        .init();

    event!(tracing::Level::WARN, "this is warn");
    event!(tracing::Level::DEBUG, "this is debug");
    let low_level_handler = LowLevelHandler::new().start();
    let scheduler = match SchedulerMutex::new(low_level_handler.clone()).await {
        Ok(scheduler) => scheduler,
        Err(e) => {
            event!(
                Level::ERROR,
                "application error with the initialize of SchedulerMutex. e -> {:?}",
                e
            );
            eprintln!("application error with the initialize of SchedulerMutex: {e}");
            process::exit(1);
        }
    };
    let conf = match get_configuration(None).await {
        Ok(conf) => conf,
        Err(e) => {
            event!(
                Level::ERROR,
                "application error with the initialize configuration of leptos. e -> {:?}",
                e
            );
            eprintln!("application error with the initialize configuration of leptos: {e}");
            process::exit(1);
        }
    };
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|| view! {  <App/> });
    event!(tracing::Level::TRACE, "the server finished configuration");
    //added the line below to register the "api" endpoint.
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .route("/checkHealth", web::get().to(check_health))
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! {  <App/> },
            )
            .service(Files::new("/", site_root))
            .wrap(Logger::default())
            .app_data(web::Data::new(low_level_handler.clone()))
            .app_data(web::Data::new(scheduler.clone()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
