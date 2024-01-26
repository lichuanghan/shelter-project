use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::{Arg, ArgMatches, Command, value_parser};
use tower_http::trace::TraceLayer;

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);
        start_tokio(port,_settings);
        // println!("TBD: start the webserver on port {}", port)
    }

    Ok(())
}
fn start_tokio(port: u16, _settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            let routes = crate::api::configure()
                .layer(TraceLayer::new_for_http());

            tracing::info!("starting axum on port {}", port);

            axum::Server::bind(&addr)
                .serve(routes.into_make_service())
                .await?;

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}