use clap::Parser;
use reqwest::{header, Client, Error, RequestBuilder};
use tokio;

/// letterboxd cli
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Token for authentication
    #[arg(short, long)]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let token = args.token;

    let client = reqwest::Client::new();
    let request = build_request_with_auth_header(&client, "http://httpbin.org/get", &token);

    // Send the request and wait for the response
    let res = request.send().await?;
    
    // Ensure the request was successful and get the response body
    let body = res.text().await?;
    
    // Print the response body
    println!("Response body: {}", body);
    
    Ok(())
}

/// Builds a request with the Authorization header
fn build_request_with_auth_header(client: &Client, url: &str, token: &str) -> RequestBuilder {
    client.get(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{self, HeaderMap};

    #[tokio::test]
    async fn test_build_request_with_auth_header() {
        let client = Client::new();
        let token = "test-token";
        let request = build_request_with_auth_header(&client, "http://example.com", token)
            .build()
            .unwrap();

        assert_eq!(request.method(), "GET");
        assert_eq!(request.url().as_str(), "http://example.com/");
        check_auth_header_exists(request.headers(), token);
    }

    fn check_auth_header_exists(headers: &HeaderMap, token: &str) {
        let auth_header = headers.get(header::AUTHORIZATION).unwrap().to_str().unwrap();
        assert_eq!(auth_header, format!("Bearer {}", token));
    }
}