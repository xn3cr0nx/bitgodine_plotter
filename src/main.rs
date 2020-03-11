#[macro_use]
extern crate log;

use crate::config::Config;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use slog::info;

mod config;
mod api_error;
mod plotter;

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().unwrap();

    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.host, config.port
    );

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || 
        App::new()
            .data(AppState {
              log: log.clone(),
            })
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .configure(plotter::routes)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(format!("{}:{}", config.host, config.port))?
        }
    };

    server.run().await
}

