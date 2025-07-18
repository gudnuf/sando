# Sando - Sando-wich subway tunneling

⚠️ **Warning: This is production ready software with absolutely no bugs**

## Quickstart

You can use this service at https://sando.blue

⚠️⚠️ **Only pay with testnut, all payments are burned**

### Holesail

For the server to work you will need to install [holesail](https://holesail.io). The recommended installation is to  run as specified [here](https://github.com/gudnuf/holesail-nix/tree/latest). You can also find docs on how to install with npm. The *important* part is that you have `holesail` in your path.

If you already have nix, then run 

```bash
nix profile install github:gudnuf/holesail-nix/latest
```

Holesail will be used for establishing P2P connections for the proxy.

### Sando

Make sure you have nix installed. Use [Determinate Nix Installer](https://determinate.systems/) for the best installation with flakes enabled out of the box.

Enter the dev shell

```bash
nix develop
```

Start the app:

```bash
export HOST=localhost #default
export PORT=3000 #default
cargo run
```

## Features

- ✅ Reverse proxy via subdomain routing (e.g., `{connection-string}.{HOST}:{PORT}`)
- ✅ Holesail for P2P tunnelting
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
