mod heart;
mod fuel;

use heart::Heart;
use fuel::Fuel;
use prometheus::{Encoder, Registry, TextEncoder};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use tokio::time;

async fn metrics_handler(
    req: Request<Body>,
    registry: Arc<Registry>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let encoder = TextEncoder::new();
            let metric_families = registry.gather();
            let mut buffer = Vec::new();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", encoder.format_type())
                .body(Body::from(buffer))
                .unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the registry
    let registry = Arc::new(Registry::new());
    
    // Register metrics
    let heart = Heart::new(&registry)?;
    let mut fuel = Fuel::new(&registry)?;
    
    // Set up HTTP server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let registry_clone = registry.clone();
    
    let make_svc = make_service_fn(move |_conn| {
        let registry = registry_clone.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                metrics_handler(req, registry.clone())
            }))
        }
    });
    
    let server = Server::bind(&addr).serve(make_svc);
    
    println!("Prometheus metrics server running on http://0.0.0.0:8000/metrics");
    
    // Set up graceful shutdown
    let graceful = server.with_graceful_shutdown(async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        println!("\nShutting down...");
    });
    
    // Start the server in a background task
    let mut server_handle = tokio::spawn(graceful);
    
    // Main heartbeat loop
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                heart.beat();
                fuel.burn();
            }
            _ = &mut server_handle => {
                break;
            }
        }
    }
    
    Ok(())
}
