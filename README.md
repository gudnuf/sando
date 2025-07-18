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
