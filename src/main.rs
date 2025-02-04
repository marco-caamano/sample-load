use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use serde::{Deserialize, Serialize};

/// Simple Prime Service
#[derive(Parser)]
#[command(version)]
struct Args {
    /// port to use
    #[arg(short, long, default_value = "9000")]
    port: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct Request {
    start: u32,
    end: u32,
}

#[derive(Serialize, Deserialize)]
struct Result {
    num: u32,
}

fn is_prime(num: u32) -> bool {
    // num is prime only if it is divisible only by 1 and the same number
    if num == 1 {
        // special case exit early
        return true;
    }
    // test
    for i in 2..num {
        if (num % i) == 0 {
            // num is divisible by i. Num is not prime
            return false;
        }
    }
    // if we didn't find an invalid case then this is a prime number
    true
}

fn calculate_primes(start: u32, end: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for i in start..=end {
        if is_prime(i) {
            result.push(i);
        }
    }

    result
}

// Handler for the POST request
async fn get_primes(request: web::Json<Request>) -> impl Responder {
    let result: Vec<u32> = calculate_primes(request.start, request.end);
    let json = match serde_json::to_string(&result) {
        Ok(data) => data,
        Err(_) => "Failed to extract data".to_string(),
    };
    HttpResponse::Ok().body(json)
}

// Main entry point to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let port = args.port.unwrap_or_default();

    println!("Starting Server on Port: {port}");

    HttpServer::new(|| {
        App::new().route("/primes", web::post().to(get_primes)) // POST /items
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
