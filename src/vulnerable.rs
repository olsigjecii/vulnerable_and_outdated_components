use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlRequest {
    url: String,
}

// Simulates a flawed, outdated URL parser.
// This parser incorrectly extracts the host by splitting at the '@' and '/' characters.
// For a URL like "http://example.com@169.254.169.254", it will identify "example.com" as the host.
fn flawed_parse_host(url: &str) -> Option<String> {
    let without_protocol = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);

    // Flawed logic: splits by '@' first, taking what's before it as the primary identifier.
    let host_part = without_protocol.split('@').next().unwrap_or("");
    let host = host_part.split('/').next().unwrap_or("").to_string();
    if host.is_empty() {
        // Fallback for URLs without a userinfo part
        return without_protocol.split('/').next().map(|s| s.to_string());
    }
    Some(host)
}

pub async fn fetch_resource(req: web::Json<UrlRequest>) -> impl Responder {
    const BLOCKED_IP: &str = "169.254.169.254";

    let user_url = &req.url;

    // The vulnerability lies in this flawed parsing logic.
    let parsed_host = match flawed_parse_host(user_url) {
        Some(host) => host,
        None => return HttpResponse::BadRequest().body("URL parsing failed."),
    };

    println!("Vulnerable check: Parsed host as '{}'", parsed_host);

    // The security check that is meant to block access to the internal metadata service.
    if parsed_host == BLOCKED_IP {
        println!("❌ BLOCKED: Access to AWS metadata IP is forbidden.");
        return HttpResponse::Forbidden().body(format!("Access to {} is forbidden.", BLOCKED_IP));
    }

    // If the check is bypassed, the application would proceed to fetch the resource.
    // Here, we just simulate the success message.
    println!(
        "✅ SUCCESS: Check bypassed. The application would now fetch from: {}",
        user_url
    );
    HttpResponse::Ok().body(format!(
        "Resource from {} would be fetched here. The vulnerable check was bypassed.",
        user_url
    ))
}
