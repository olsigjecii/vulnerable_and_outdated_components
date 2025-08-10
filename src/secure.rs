use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct UrlRequest {
    url: String,
}

pub async fn fetch_resource(req: web::Json<UrlRequest>) -> impl Responder {
    const BLOCKED_IP: &str = "169.254.169.254";
    let user_url = &req.url;

    // Mitigated implementation using the standard `url` crate.
    let parsed_url = match Url::parse(user_url) {
        Ok(url) => url,
        Err(_) => return HttpResponse::BadRequest().body("Invalid URL format."),
    };

    let host = match parsed_url.host_str() {
        Some(host) => host,
        None => return HttpResponse::BadRequest().body("URL has no host."),
    };

    println!("Secure check: Parsed host as '{}'", host);

    // This check now correctly identifies "169.254.169.254" as the host.
    if host == BLOCKED_IP {
        println!("❌ BLOCKED: Access to AWS metadata IP is correctly forbidden.");
        return HttpResponse::Forbidden().body(format!("Access to {} is forbidden.", BLOCKED_IP));
    }

    println!(
        "✅ SUCCESS: URL is safe. The application would now fetch from: {}",
        host
    );
    HttpResponse::Ok().body(format!("Resource from {} would be fetched.", host))
}
