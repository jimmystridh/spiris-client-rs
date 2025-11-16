# Spiris Bokföring och Fakturering API Client for Rust

[![Crates.io](https://img.shields.io/crates/v/spiris_bokforing.svg)](https://crates.io/crates/spiris_bokforing)
[![Documentation](https://docs.rs/spiris_bokforing/badge.svg)](https://docs.rs/spiris_bokforing)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A comprehensive Rust client library for the [Spiris Bokföring och Fakturering API](https://developer.visma.com/api/eaccounting) (formerly Visma eAccounting).

## Features

- **OAuth2 Authentication**: Complete OAuth2 flow support with PKCE and token refresh
- **Type-safe API**: Strongly typed request/response models
- **Async/Await**: Built on tokio and reqwest for async operations
- **Automatic Retries**: Exponential backoff for transient failures
- **Rate Limiting**: Automatic handling of API rate limits (600 req/min)
- **Configurable**: Builder patterns for client and retry configuration
- **Observability**: Optional tracing support for request/response logging
- **Comprehensive Coverage**: Support for customers, invoices, articles, and more
- **Error Handling**: Rich error types with detailed information
- **Production Ready**: CI/CD, comprehensive tests, and battle-tested
- **Well-documented**: Extensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
spiris_bokforing = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use spiris_bokforing::{Client, AccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an access token (usually obtained via OAuth2)
    let token = AccessToken::new("your_access_token".to_string(), 3600, None);

    // Create the API client
    let client = Client::new(token);

    // List customers
    let customers = client.customers().list(None).await?;
    println!("Found {} customers", customers.data.len());

    Ok(())
}
```

## Authentication

The Spiris API uses OAuth2 for authentication. Here's how to authenticate:

### 1. Register Your Application

First, register your application in the [Visma Developer Portal](https://developer.visma.com/) to obtain:
- Client ID
- Client Secret
- Redirect URI

### 2. Implement OAuth2 Flow

```rust
use spiris_bokforing::auth::{OAuth2Config, OAuth2Handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OAuth2Config::new(
        "your_client_id".to_string(),
        "your_client_secret".to_string(),
        "http://localhost:8080/callback".to_string(),
    );

    let handler = OAuth2Handler::new(config)?;

    // Get authorization URL
    let (auth_url, csrf_token, pkce_verifier) = handler.authorize_url();
    println!("Visit this URL to authorize: {}", auth_url);

    // After user authorizes and you receive the code...
    let token = handler.exchange_code(code, pkce_verifier).await?;

    // Use the token to create a client
    let client = Client::new(token);

    Ok(())
}
```

## Usage Examples

### List Customers with Pagination

```rust
use spiris_bokforing::{Client, AccessToken, PaginationParams};

let token = AccessToken::new("your_token".to_string(), 3600, None);
let client = Client::new(token);

let params = PaginationParams::new().page(0).pagesize(100);
let customers = client.customers().list(Some(params)).await?;

for customer in customers.data {
    println!("{:?}: {:?}", customer.id, customer.name);
}
```

### Create a Customer

```rust
use spiris_bokforing::{Client, Customer, Address};

let new_customer = Customer {
    name: Some("Acme Corporation".to_string()),
    email: Some("contact@acme.com".to_string()),
    phone: Some("+46123456789".to_string()),
    invoice_address: Some(Address {
        address1: Some("123 Main Street".to_string()),
        city: Some("Stockholm".to_string()),
        postal_code: Some("11122".to_string()),
        country_code: Some("SE".to_string()),
        ..Default::default()
    }),
    is_active: Some(true),
    payment_terms_in_days: Some(30),
    ..Default::default()
};

let created = client.customers().create(&new_customer).await?;
println!("Created customer with ID: {:?}", created.id);
```

### Create an Invoice

```rust
use spiris_bokforing::{Client, Invoice, InvoiceRow};
use chrono::Utc;

let invoice = Invoice {
    customer_id: Some("customer-id-here".to_string()),
    invoice_date: Some(Utc::now()),
    currency_code: Some("SEK".to_string()),
    rows: vec![
        InvoiceRow {
            text: Some("Consulting services".to_string()),
            unit_price: Some(1000.0),
            quantity: Some(10.0),
            ..Default::default()
        }
    ],
    ..Default::default()
};

let created_invoice = client.invoices().create(&invoice).await?;
println!("Invoice #{:?} created", created_invoice.invoice_number);
```

### Search with Filters

```rust
use spiris_bokforing::QueryParams;

let query = QueryParams::new()
    .filter("IsActive eq true")
    .select("Id,Name,Email");

let active_customers = client.customers().search(query, None).await?;
```

### Manage Articles/Products

```rust
use spiris_bokforing::Article;

let article = Article {
    name: Some("Consulting Hour".to_string()),
    unit: Some("hours".to_string()),
    sales_price: Some(1200.0),
    is_active: Some(true),
    ..Default::default()
};

let created_article = client.articles().create(&article).await?;
```

## API Coverage

The library currently supports the following endpoints:

- **Customers**: List, Get, Create, Update, Delete, Search
- **Invoices**: List, Get, Create, Update, Delete, Search
- **Articles**: List, Get, Create, Update, Delete, Search

More endpoints will be added in future releases.

## Error Handling

The library provides comprehensive error handling:

```rust
use spiris_bokforing::Error;

match client.customers().get("invalid-id").await {
    Ok(customer) => println!("Found customer: {:?}", customer.name),
    Err(Error::NotFound(msg)) => println!("Customer not found: {}", msg),
    Err(Error::TokenExpired) => println!("Token expired, please refresh"),
    Err(Error::RateLimitExceeded(msg)) => println!("Rate limit hit: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

## Rate Limiting

The Spiris API has a rate limit of **600 requests per minute** per client per endpoint. The library automatically handles rate limit errors and returns appropriate error types.

## Token Expiration and Refresh

Access tokens expire after 1 hour. The library checks token expiration before making requests and provides built-in token refresh:

```rust
use spiris_bokforing::auth::{OAuth2Config, OAuth2Handler};

// Check if token is expired
if client.is_token_expired() {
    let current_token = client.get_access_token();

    if let Some(refresh_token) = current_token.refresh_token {
        // Use the OAuth2 handler to refresh the token
        let config = OAuth2Config::new(/* ... */);
        let handler = OAuth2Handler::new(config)?;
        let new_token = handler.refresh_token(refresh_token).await?;

        // Update the client with the new token
        client.set_access_token(new_token);
    }
}
```

## Advanced Configuration

The client supports extensive configuration for production use:

```rust
use spiris_bokforing::{Client, AccessToken, ClientConfig, RetryConfig};
use std::time::Duration;

let token = AccessToken::new("token".to_string(), 3600, None);

// Configure retry behavior
let retry_config = RetryConfig::new()
    .max_retries(5)
    .initial_interval(Duration::from_secs(1))
    .max_interval(Duration::from_secs(30));

// Create client with custom configuration
let config = ClientConfig::new()
    .base_url("https://eaccountingapi.vismaonline.com/v2/")
    .timeout_seconds(60)
    .retry_config(retry_config)
    .enable_tracing(true);

let client = Client::with_config(token, config);
```

## Retry Logic

The client automatically retries failed requests with exponential backoff:

- **Network errors**: Automatically retried
- **Rate limits (429)**: Automatically retried with backoff
- **Server errors (5xx)**: Automatically retried
- **Client errors (4xx)**: Not retried (permanent errors)

Configure retry behavior:

```rust
let retry_config = RetryConfig::new()
    .max_retries(3)                                  // Max retry attempts
    .initial_interval(Duration::from_millis(500))    // Initial backoff
    .max_interval(Duration::from_secs(30))           // Max backoff
    .multiplier(2.0);                                // Backoff multiplier
```

## Examples

The `examples/` directory contains complete working examples:

- `oauth_flow.rs`: OAuth2 authentication flow
- `list_customers.rs`: List customers with pagination
- `create_customer.rs`: Create a new customer
- `create_invoice.rs`: Create an invoice

Run an example with:

```bash
export SPIRIS_ACCESS_TOKEN="your_token_here"
cargo run --example list_customers
```

## Testing

Run the test suite:

```bash
cargo test
```

Run specific tests:

```bash
# Run only library tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run with output
cargo test -- --nocapture
```

## Performance Tips

### Connection Pooling

The client uses reqwest's built-in connection pooling. Reuse the same `Client` instance for multiple requests:

```rust
// Good: Reuse client
let client = Client::new(token);
for customer_id in customer_ids {
    let customer = client.customers().get(customer_id).await?;
}

// Bad: Creating new client for each request
for customer_id in customer_ids {
    let client = Client::new(token.clone());
    let customer = client.customers().get(customer_id).await?;
}
```

### Batch Operations

When possible, use pagination to fetch multiple records in one request:

```rust
// Fetch 100 customers at once instead of 100 individual requests
let params = PaginationParams::new().pagesize(100);
let customers = client.customers().list(Some(params)).await?;
```

### Timeout Configuration

Adjust timeouts based on your network conditions:

```rust
let config = ClientConfig::new()
    .timeout_seconds(60)  // Increase for slower networks
    .retry_config(
        RetryConfig::new()
            .max_retries(5)
            .initial_interval(Duration::from_secs(2))
    );
```

## Security Best Practices

### Never Hardcode Credentials

Always use environment variables or secure configuration management:

```rust
// Good
let token = std::env::var("SPIRIS_ACCESS_TOKEN")?;

// Bad - Never do this!
// let token = "hardcoded_token_12345";
```

### Token Storage

Store refresh tokens securely:

```rust
use std::fs;
use std::os::unix::fs::PermissionsExt;

// Write token to file with restricted permissions
let token_json = serde_json::to_string(&token)?;
fs::write(".spiris_token", token_json)?;
fs::set_permissions(".spiris_token", fs::Permissions::from_mode(0o600))?;
```

### HTTPS Only

The client uses HTTPS by default. Never modify the base URL to use HTTP:

```rust
// The default is already HTTPS - don't change it
const DEFAULT_BASE_URL: &str = "https://eaccountingapi.vismaonline.com/v2/";
```

## Troubleshooting

### Token Expired Errors

If you're getting `TokenExpired` errors:

```rust
// Check token expiration before making requests
if client.is_token_expired() {
    // Refresh the token
    let current_token = client.get_access_token();
    if let Some(refresh_token) = current_token.refresh_token {
        let handler = OAuth2Handler::new(oauth_config)?;
        let new_token = handler.refresh_token(refresh_token).await?;
        client.set_access_token(new_token);
    }
}
```

### Rate Limiting

If you're hitting rate limits (600 requests/minute):

```rust
// Configure more aggressive retry backoff
let retry_config = RetryConfig::new()
    .max_retries(10)
    .initial_interval(Duration::from_secs(5))
    .max_interval(Duration::from_secs(60));

let config = ClientConfig::new().retry_config(retry_config);
let client = Client::with_config(token, config);
```

### Network Timeouts

For unreliable networks:

```rust
let config = ClientConfig::new()
    .timeout_seconds(120)  // 2 minutes
    .retry_config(
        RetryConfig::new()
            .max_retries(5)
            .initial_interval(Duration::from_secs(2))
    );
```

### Debugging API Requests

Enable tracing to see detailed request/response information:

```rust
// Add to Cargo.toml
// tracing-subscriber = "0.3"

use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = ClientConfig::new().enable_tracing(true);
    let client = Client::with_config(token, config);

    // Now you'll see detailed logs
}
```

## FAQ

### Q: Do I need to manually refresh tokens?

A: The client checks token expiration before each request and returns a `TokenExpired` error if the token is expired. You can either:
1. Manually refresh using `OAuth2Handler::refresh_token()`
2. Implement automatic refresh logic in your application

### Q: What's the rate limit?

A: The API has a rate limit of **600 requests per minute** per client per endpoint. The client automatically retries rate-limited requests with exponential backoff.

### Q: Can I use this with multiple accounts?

A: Yes! Create separate `Client` instances for each account with different access tokens:

```rust
let client1 = Client::new(token1);
let client2 = Client::new(token2);
```

### Q: How do I handle pagination for large datasets?

A: Use a loop to fetch all pages:

```rust
let mut all_customers = Vec::new();
let mut page = 0;
let pagesize = 100;

loop {
    let params = PaginationParams::new().page(page).pagesize(pagesize);
    let response = client.customers().list(Some(params)).await?;

    all_customers.extend(response.data);

    if !response.meta.has_next_page {
        break;
    }
    page += 1;
}
```

### Q: What happens if my API call fails?

A: The client automatically retries transient failures (network errors, rate limits, 5xx errors) with exponential backoff. Permanent errors (4xx) are returned immediately.

### Q: Can I customize the retry behavior?

A: Yes! See the "Retry Logic" section for configuration options.

### Q: Is this thread-safe?

A: Yes! The `Client` can be safely cloned and shared across threads:

```rust
let client = Client::new(token);

// Clone for use in different threads
let client1 = client.clone();
let client2 = client.clone();

tokio::spawn(async move {
    client1.customers().list(None).await
});

tokio::spawn(async move {
    client2.invoices().list(None).await
});
```

### Q: What's the difference between Spiris and Visma eAccounting?

A: Spiris Bokföring och Fakturering is the new name for Visma eAccounting. All API endpoints and functionality remain exactly the same - only the branding has changed.

## Migration Guide

### From visma_eaccounting to spiris_bokforing

If you were using an earlier version with the `visma_eaccounting` package name:

1. Update `Cargo.toml`:
```toml
[dependencies]
# Old
# visma_eaccounting = "0.1.0"

# New
spiris_bokforing = "0.1.0"
```

2. Update imports:
```rust
// Old
use visma_eaccounting::{Client, AccessToken};

// New
use spiris_bokforing::{Client, AccessToken};
```

3. Update environment variables:
```bash
# Old
export VISMA_ACCESS_TOKEN="..."
export VISMA_CLIENT_ID="..."

# New
export SPIRIS_ACCESS_TOKEN="..."
export SPIRIS_CLIENT_ID="..."
```

All API functionality remains identical - no code changes needed beyond the import statements.

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

Browse available modules:
- `spiris_bokforing::auth` - OAuth2 authentication
- `spiris_bokforing::client` - HTTP client
- `spiris_bokforing::endpoints` - API endpoints
- `spiris_bokforing::error` - Error types
- `spiris_bokforing::types` - Data models
- `spiris_bokforing::retry` - Retry configuration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/jimmystridh/claude_jungle_bamboo
cd claude_jungle_bamboo

# Run tests
cargo test

# Run examples (requires API credentials)
export SPIRIS_ACCESS_TOKEN="your_token"
cargo run --example list_customers

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build documentation
cargo doc --no-deps
```

### Reporting Issues

When reporting issues, please include:
- Rust version (`rustc --version`)
- Package version
- Minimal code example
- Error messages
- Expected vs actual behavior

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

- [Spiris Bokföring och Fakturering API Documentation](https://developer.visma.com/api/eaccounting)
- [Visma Developer Portal](https://developer.visma.com/)
- [API Authentication Guide](https://developer.vismaonline.com/docs/authentication)
- [Visma Community Forum](https://community.visma.com/t5/Visma-eAccounting-API/ct-p/IN_MA_eAccountingAPI)

## Note

Spiris Bokföring och Fakturering was formerly known as Visma eAccounting. All API endpoints and technical details remain the same.

## Disclaimer

This is an unofficial client library and is not affiliated with or endorsed by Visma or Spiris. Use at your own risk.
