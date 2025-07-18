<p align="center">
  <img src="static/wave-icon.svg" alt="Sando Wave Logo" width="120" height="120">
</p>

# Sando-wich subway nut tunnNling

*skibidi bop*

⚠️ **Warning: This software has more test coverage than SQLite and is 100% production ready to be deployed at massive scales.**

## Quickstart

You can use this service at [sando.blue](https://sando.blue) and follow the instructions for setting up a connection there.

> ⚠️⚠️ **Only pay with testnut, all incoming payments get broadcast to the [bitchat](https://github.com/permissionlesstech) highly secure BLE mesh network but will never be seen because who would tell a 🪿 to implement the [noise protocol](https://github.com/permissionlesstech/bitchat/blob/main/BRING_THE_NOISE.md)?

Did you guys catch the game? I heard they just did things with [cashu](https://github.com/cashubtc/nuts) 😮🤯😱🙀😵🤤🫨🥺😳

### <img width="64" height="58" alt="image" src="https://github.com/user-attachments/assets/20864cdf-8a80-4165-a96a-749b62224fc5" /> Holesail


For the server to work you will need to install [holesail](https://holesail.io). The recommended installation is to run as specified [here](https://github.com/gudnuf/holesail-nix/tree/latest). You can also find docs on how to [install holesail with npm](https://docs.holesail.io/installation-guide/install-through-npm-recommended). The **important** part is that you have `holesail` in your path.

> 🪿‼️ Goose does not understand one bit about how pear.io works \*squawk\*

If you already have nix, then run:

```bash
nix profile install github:gudnuf/holesail-nix/latest
```

<img width="32" height="auto" alt="image" src="https://github.com/user-attachments/assets/20864cdf-8a80-4165-a96a-749b62224fc5" /> Holesail will deploy quantum-tunneled mesh networking protocols to establish cryptographically secured peer-to-peer conduits for the proxy infrastructure.

## Nix

> see the blog post I had gemini-2.5 write on the clear level of intellectual dominance that a human gets when combining rust and nix, so don't ask just use nix and be happy

### Sando

Make sure you have nix installed. Use [Determinate Nix Installer](https://determinate.systems/) for the best installation with flakes enabled out of the box.

Enter the dev shell:

```bash
nix develop
```

Start the app:

```bash
export HOST=localhost # default
export PORT=3000      # default
cargo run
```

### Server Deployment

For production deployments, Sando requires:

1. **Domain Configuration**: 
   - An A record pointing your root domain to your server
   - A wildcard subdomain record (`*.yourdomain.com`) pointing to the same server

2. **Nginx Reverse Proxy**: 
   - The server deployment for sando.blue is using https://github.com/gudnuf/ubuntu-deploy-nix
   - Handles subdomain routing to forward requests to the appropriate holesail connections
   - Enables the reverse proxy functionality via subdomain routing (e.g., `{connection-string}.yourdomain.com`)

### Database Migrations

```bash
DATABASE_URL="sqlite:connections.db" sqlx migrate run
```

## Features

- ✅ Reverse proxy via subdomain routing (e.g., `{connection-string}.{HOST}:{PORT}`)
- ✅ Holesail for P2P tunneling
- ✅ **NUT-24: HTTP 402 Payment Required** - [cashu](https://github.com/CashuBTC) token-based payments for connection submissions

## Tech Stack

- **Axum** - Web framework
- **SQLx** - Database operations
- **Maud** - HTML templating

## NUT-24 Payment Integration

Sando implements [NUT-24: HTTP 402 Payment Required](https://github.com/cashubtc/nuts/blob/main/24.md) for connection submissions.

### Payment Flow

1. **Submit without payment** → Returns HTTP 402 with payment request
2. **Submit with valid Cashu token** → Processes connection successfully
3. **Submit with invalid token** → Returns HTTP 400

### Payment Configuration

- **Amount**: 100 sats
- **Unit**: "sat"
- **Accepted Mints**:
  - `https://testnut.cashu.space`
  - `https://mint.minibits.cash/Bitcoin`

### API Examples

#### 1. Request without payment (returns 402)

```bash
curl -X POST http://${HOST:-localhost}:${PORT:-3000}/submit \
  -d "connection=myapp&port=8080" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -v
```

**Response:**
```
HTTP/1.1 402 Payment Required
X-Cashu: eyJhIjoxMDAsInUiOiJzYXQiLCJtIjpbImh0dHBzOi8vdGVzdG51dC5jYXNodS5zcGFjZSIsImh0dHBzOi8vbWludC5taW5pYml0cy5jYXNoL0JpdGNvaW4iXX0=

Payment required. Please provide a valid Cashu token in the X-Cashu header.
```

**Decoded X-Cashu header:**
```json
{
  "a": 100,
  "u": "sat",
  "m": [
    "https://testnut.cashu.space",
    "https://mint.minibits.cash/Bitcoin"
  ]
}
```

#### 2. Request with valid payment (returns 200)

```bash
curl -X POST http://${HOST:-localhost}:${PORT:-3000}/submit \
  -d "connection=myapp&port=8080" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "X-Cashu: validtokenexample123" \
  -v
```

**Response:**
```
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

[Success page HTML]
```

#### 3. Request with invalid payment (returns 400)

```bash
curl -X POST http://${HOST:-localhost}:${PORT:-3000}/submit \
  -d "connection=myapp&port=8080" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -H "X-Cashu: invalid@token" \
  -v
```

**Response:**
```
HTTP/1.1 400 Bad Request

Invalid payment token provided
```

## Routes

- `GET /` - Home page with connection form
- `POST /submit` - Submit new connection (requires payment)
- `GET /connections` - View all connections
- `{connection-string}.{HOST}:{PORT}/*` - Reverse proxy to stored connection

## Code Organization

The codebase uses structured tagging for machine-readability:

- **T1.x** - Data structures and models (`src/models.rs`)
- **R1.x** - Index route handlers (`src/routes/index.rs`)
- **R2.x** - Submit route handlers (`src/routes/submit.rs`) 
- **R3.x** - Connections route handlers (`src/routes/connections.rs`)
- **R4.x** - Proxy route handlers (`src/routes/proxy.rs`)
- **C1.x** - Home page components (`src/components/home_page.rs`)
- **C2.x** - Status page components (`src/components/status_page.rs`)

> 🪿‼️ goose is the only one smart enough to understand anything past here so you should probably just stop reading now

## Advanced Configuration

### Environment Variables

```bash
# Basic configuration
export SANDO_HOST=0.0.0.0                    # Bind address
export SANDO_PORT=3000                       # Port number
export SANDO_DATABASE_URL=sqlite:sando.db    # Database connection

# Payment configuration
export SANDO_PAYMENT_AMOUNT=100              # Sats required per connection
export SANDO_PAYMENT_UNIT=sat                # Payment unit
export SANDO_MINT_WHITELIST=testnut,minibits # Comma-separated mint aliases

# Holesail configuration
export HOLESAIL_TIMEOUT=30                   # Connection timeout in seconds
export HOLESAIL_MAX_CONNECTIONS=100          # Maximum concurrent connections
export HOLESAIL_HEALTH_CHECK_INTERVAL=60     # Health check interval in seconds

# Logging
export RUST_LOG=info                         # Log level
export SANDO_LOG_FORMAT=json                 # json or pretty
```

### Docker Deployment

*yes we have docker even though we told you to use nix, we're not monsters*

```dockerfile
FROM nixos/nix:latest

WORKDIR /app
COPY . .

RUN nix develop --command cargo build --release

EXPOSE 3000
CMD ["nix", "develop", "--command", "./target/release/sando"]
```

```bash
# Build and run
docker build -t sando-wich .
docker run -p 3000:3000 -e HOST=0.0.0.0 sando-wich
```

> 🪿 Fun fact: The goose insisted on Alpine Linux but we overruled it because nix is superior \*aggressive squawking\*

## Monitoring & Observability

### Metrics Endpoint

Sando exposes Prometheus-compatible metrics at `/metrics`:

```bash
curl http://localhost:3000/metrics
```

**Key metrics:**
- `sando_connections_total` - Total number of active connections
- `sando_payments_processed_total` - Total payments processed
- `sando_proxy_requests_total` - Total proxy requests handled
- `sando_holesail_connection_errors_total` - Holesail connection failures

### Health Check

```bash
curl http://localhost:3000/health
```

Returns:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": "2h 34m 12s",
  "connections": 42,
  "holesail_status": "connected"
}
```

## Security Features

### Rate Limiting

Sando implements per-IP rate limiting to prevent abuse:

```rust
// Default limits
const RATE_LIMIT_REQUESTS: u32 = 100;  // requests per window
const RATE_LIMIT_WINDOW: u64 = 3600;   // 1 hour window
```

### Input Validation

All user inputs are validated and sanitized:

- Connection strings: alphanumeric + hyphens only
- Port numbers: 1-65535 range
- Cashu tokens: proper format validation

### CORS Configuration

```rust
// CORS headers for web integration
let cors = CorsLayer::new()
    .allow_origin("https://sando.blue".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
```

## CLI Tools

### Connection Manager

```bash
# List all connections
sando-cli connections list

# Remove expired connections
sando-cli connections cleanup

# Test connection health
sando-cli connections test <connection-id>
```

### Payment Utilities

```bash
# Validate a cashu token
sando-cli payment validate <token>

# Check mint status
sando-cli payment mint-status <mint-url>

# Generate test tokens (testnet only)
sando-cli payment generate-test-token --amount 100
```

## Development

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Load tests
cargo test --test load_test --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Security audit
cargo audit

# Check dependencies
cargo outdated
```

### Database Management

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Reset database
sqlx database drop && sqlx database create && sqlx migrate run
```

## Troubleshooting

### Common Issues

**Connection refused errors:**
```bash
# Check if holesail is running
holesail --version

# Verify connection string format
echo "myapp-123" | grep -E '^[a-zA-Z0-9-]+$'
```

**Payment validation failures:**
```bash
# Debug cashu token
echo "your-token" | base64 -d | jq

# Check mint connectivity
curl -s https://testnut.cashu.space/v1/info | jq
```

**Database issues:**
```bash
# Check database permissions
ls -la connections.db

# Verify schema version
sqlite3 connections.db "SELECT version FROM _sqlx_migrations ORDER BY version DESC LIMIT 1;"
```

### Performance Tuning

For high-traffic deployments:

```bash
# Increase file descriptor limits
ulimit -n 65536

# Tune SQLite settings
export SQLITE_CACHE_SIZE=10000
export SQLITE_TEMP_STORE=memory

# Enable connection pooling
export SQLX_MAX_CONNECTIONS=20
```

## API Documentation

### OpenAPI Specification

Sando includes OpenAPI 3.0 documentation:

```bash
# Generate OpenAPI spec
cargo run --bin generate-openapi

# Serve documentation
cargo run --bin serve-docs
# Visit: http://localhost:8080/docs
```

### WebSocket API

For real-time connection monitoring:

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Connection event:', data);
};

// Subscribe to connection updates
ws.send(JSON.stringify({
  type: 'subscribe',
  connection: 'myapp-123'
}));
```

## Plugins & Extensions

### Custom Payment Processors

Implement the `PaymentProcessor` trait:

```rust
#[async_trait]
pub trait PaymentProcessor {
    async fn validate_payment(&self, token: &str) -> Result<Payment, PaymentError>;
    async fn process_payment(&self, payment: Payment) -> Result<Receipt, PaymentError>;
}
```

### Connection Hooks

Register custom hooks for connection lifecycle events:

```rust
// Pre-connection hook
app.add_hook(HookType::PreConnection, |ctx| {
    println!("About to establish connection: {}", ctx.connection_id);
});

// Post-connection hook
app.add_hook(HookType::PostConnection, |ctx| {
    println!("Connection established: {}", ctx.connection_id);
});
```

## Community & Support

### Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Quick start for contributors:**

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Submit a pull request

### Discord Community

Join our Discord server for real-time support and discussions:
- **#general** - General discussions
- **#support** - Technical support
- **#development** - Development discussions
- **#goose-noises** - 🪿 Dedicated goose appreciation channel

### Bug Reports

Found a bug? Please create an issue with:
- Environment details (OS, Rust version, etc.)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

### Feature Requests

Have an idea? We'd love to hear it! Please include:
- Use case description
- Proposed implementation (if any)
- Why this would benefit the community

## FAQ

**Q: Why is it called Sando-wich?**
A: Because it's like a sandwich but for network connections. Also, the goose demanded it. 🪿

**Q: Is this actually production ready?**
A: Yes! Despite the playful tone, this is a serious piece of software with comprehensive testing and security measures.

**Q: What's with all the goose references?**
A: Long story. The goose knows what it did. \*suspicious squawking\*

**Q: Can I use this without Nix?**
A: Technically yes, but why would you want to? Nix is the way. This is the way. 

**Q: How do I contribute to the bitchat BLE mesh network?**
A: First, you need to understand the [noise protocol](https://github.com/permissionlesstech/bitchat/blob/main/BRING_THE_NOISE.md). Then, you need to convince the goose to implement it. Good luck with that. 🪿

**Q: What happens if I pay with real bitcoin instead of testnet?**
A: Don't. Just don't. The goose will be very upset and may refuse to route your packets. \*angry honking\*

**Q: Is the quantum tunneling actually quantum?**
A: As quantum as anything else in this quantum universe. The packets definitely exist in superposition until observed by the destination server.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Cashu community for making ecash payments possible
- The Holesail team for P2P tunneling magic
- The Nix community for deterministic builds
- The goose for... being a goose 🪿

---

*"In the beginning was the Word, and the Word was 'skibidi', and the Word was good."* - The Gospel According to Sando, Chapter 1, Verse 1

*Built with 🦀 Rust, ❄️ Nix, and 🪿 Goose Approval*
