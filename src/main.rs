#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix::prelude::*;
    use actix_files::Files;
    use actix_web::middleware::Logger;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    // use leptos_start::app::ChangeCronString;
    use leptos_start::app::CheckPump;
    use leptos_start::app::PumpWater;
    use leptos_start::app::*;
    use leptos_start::my_scheduler::*;
    use leptos_start::utils::LowLevelHandler;
    use tracing::info;
    // let low_level_handler = LowLevelHandler { pump_relay_pin: 4 }.start();
    let low_level_handler = LowLevelHandler::new().start();
    let file_appender = tracing_appender::rolling::daily("./logs", "log_of_day");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();
    info!("started the server");
    let scheduler = SchedulerMutex::new(low_level_handler.clone())
        .await
        .unwrap();
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let _ = PumpWater::register();
    let _ = CheckPump::register();
    let _ = ChangeCronString::register();
    let _ = CancelPump::register();
    let routes = generate_route_list(|cx| view! { cx, <App/> });
    //added the line below to register the "api" endpoint.
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
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
