use std::sync::Arc;

use geo::GeoIndex;
use poem::{
    get, handler,
    listener::TcpListener,
    middleware::Tracing,
    web::{Data, Json, Query},
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
    /// Cached geo-index for faster load time
    #[arg(short, long, env)]
    cache: Option<String>,

    /// Path to pbf file
    #[arg(short, long, env)]
    pbf: String,
}

mod geo;

#[derive(serde::Serialize)]
struct Response<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
}

#[handler]
fn query(
    data: Data<&Arc<GeoIndex>>,
    Query(query): Query<QueryParams>,
) -> Json<Response<AddressResponse>> {
    if let Some(address) = data.0.find(query.lat, query.lon) {
        Json(Response {
            success: true,
            data: Some(AddressResponse { address }),
            error: None,
        })
    } else {
        Json(Response {
            success: false,
            data: None,
            error: Some("No address found".to_string()),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let geo = match args.cache {
        Some(path) => {
            //check if file path exists
            match std::fs::File::open(&path) {
                Ok(file) => {
                    let start = std::time::Instant::now();
                    println!("load index from file");
                    let geo: GeoIndex = bincode::deserialize_from(file).unwrap();
                    println!("Loaded index in {}ms", start.elapsed().as_millis());
                    geo
                }
                Err(_e) => {
                    println!("cannot load index => rebuild");
                    let mut geo = GeoIndex::new();
                    geo.build(&args.pbf);
                    // save geo to file
                    std::fs::write(&path, bincode::serialize(&geo).unwrap())
                        .expect("Unable to write file");
                    geo
                }
            }
        }
        None => {
            let mut geo = GeoIndex::new();
            geo.build(&args.pbf);
            geo
        }
    };

    let app = Route::new()
        .at("/query", get(query))
        .data(Arc::new(geo))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("Fast-pbf-server")
        .run(app)
        .await
}
