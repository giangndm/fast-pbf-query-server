use std::sync::Arc;

use geo::GeoIndex;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::Tracing,
    web::{Data, Path, Query},
    EndpointExt, Route, Server,
};

use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QueryParams {
    lat: f32,
    lon: f32,
}

/// Pbf query server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to pbf file
    #[arg(short, long, env)]
    path: String,
}

mod geo;

#[handler]
fn query(data: Data<&Arc<GeoIndex>>, Query(query): Query<QueryParams>) -> String {
    if let Some(address) = data.0.find(query.lat, query.lon) {
        format!("Address: {}", address)
    } else {
        "Not found".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let mut geo = GeoIndex::new();
    geo.load(&args.path);

    let app = Route::new()
        .at("/query", get(query))
        .data(Arc::new(geo))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
