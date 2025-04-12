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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), true); // 1 is considered prime in this implementation
        assert_eq!(is_prime(2), true); // 2 is prime
        assert_eq!(is_prime(3), true); // 3 is prime
        assert_eq!(is_prime(4), false); // 4 is not prime
        assert_eq!(is_prime(17), true); // 17 is prime
        assert_eq!(is_prime(18), false); // 18 is not prime
    }

    #[test]
    fn test_calculate_primes() {
        assert_eq!(calculate_primes(1, 10), vec![1, 2, 3, 5, 7]); // primes between 1 and 10
        assert_eq!(calculate_primes(10, 20), vec![11, 13, 17, 19]); // primes between 10 and 20
        assert_eq!(calculate_primes(20, 30), vec![23, 29]); // primes between 20 and 30
        assert_eq!(calculate_primes(30, 30), Vec::<u32>::new()); // explicitly specify the type of the empty vector
    }
}


#[cfg(test)]
mod tests_api {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_primes_api() {
        let app = test::init_service(
            App::new().route("/primes", web::post().to(get_primes)),
        )
        .await;

        let request = test::TestRequest::post()
            .uri("/primes")
            .set_json(&Request { start: 1, end: 10 })
            .to_request();

        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());

        let body: Vec<u32> = test::read_body_json(response).await;
        assert_eq!(body, vec![1, 2, 3, 5, 7]); // primes between 1 and 10
    }
}
