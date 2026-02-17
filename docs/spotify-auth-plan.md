# Spotify TUI Authentication Plan

## Overview

Implement PKCE (Proof Key for Code Exchange) authentication flow to connect a terminal UI application to Spotify's Web API.

## Prerequisites

- Register app at [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
- Get `client_id` and configure `redirect_uri` (e.g., `http://127.0.0.1:8080/callback`)

---

## Implementation Steps

### 1. PKCE Generation (Partially Done)

**Status:** Partial - utilities exist, need to combine

**Existing modules:**
- `random_string.rs` → generates 64-character code verifier
- `sha256.rs` → hashes verifier with SHA256
- `base64.rs` → base64url-encodes hash (no padding)

**Needed:**
- Combine into single `generate_pkce_challenge()` function that returns:
  - `code_verifier` (random string)
  - `code_challenge` (SHA256 hash, base64url encoded)

---

### 2. Local Callback Server

**Purpose:** Receive OAuth redirect with authorization code

**Implementation:**
- Start HTTP server on `127.0.0.1:8080` (or random port)
- Listen for GET request to `/callback?code=...&state=...`
- Extract `code` parameter from query string
- Return simple HTML response to close browser or show success

**Options:**
- `tiny_http` - minimal, synchronous
- `tokio` + `axum` - async, more features
- `std::net::TcpListener` - raw, maximum control

---

### 3. Open Browser for Authorization

**Purpose:** Let user grant permissions to the app

**Implementation:**
1. Construct authorization URL:
   ```
   https://accounts.spotify.com/authorize?
     client_id=YOUR_CLIENT_ID&
     response_type=code&
     redirect_uri=http://127.0.0.1:8080/callback&
     code_challenge_method=S256&
     code_challenge=<YOUR_CODE_CHALLENGE>&
     scope=user-read-private user-read-email&
     state=<RANDOM_STATE>
   ```

2. Open URL in user's default browser using `open` crate

3. Store `code_verifier` and `state` for later validation

**Common Scopes:**
- `user-read-private` - read user profile
- `user-read-email` - read email
- `user-read-playback-state` - see what's playing
- `user-modify-playback-state` - control playback
- `playlist-read-private` - read private playlists

---

### 4. Exchange Code for Access Token

**Purpose:** Trade authorization code for access/refresh tokens

**Implementation:**
- POST to `https://accounts.spotify.com/api/token`
- Body (form-urlencoded):
  ```
  grant_type=authorization_code
  code=<received_code>
  redirect_uri=http://127.0.0.1:8080/callback
  client_id=YOUR_CLIENT_ID
  code_verifier=<stored_code_verifier>
  ```
- Headers: `Content-Type: application/x-www-form-urlencoded`

**Response:**
```json
{
  "access_token": "...",
  "token_type": "Bearer",
  "scope": "...",
  "expires_in": 3600,
  "refresh_token": "..."
}
```

---

### 5. Token Storage

**Purpose:** Persist tokens between app sessions

**Implementation:**
- Store in `~/.config/tuify/tokens.json` (Linux/Mac) or `%APPDATA%/tuify/tokens.json` (Windows)
- Use `dirs` crate for cross-platform config directory
- Structure:
  ```json
  {
    "access_token": "...",
    "refresh_token": "...",
    "expires_at": 1234567890,
    "token_type": "Bearer"
  }
  ```

---

### 6. TUI State Management

**States:**

1. **Unauthenticated**
   - Show "Not logged in"
   - Prompt: "Press Enter to login with Spotify"
   - On Enter: Start auth flow

2. **Authenticating**
   - Show "Opening browser..."
   - Show "Waiting for authorization..."
   - Start local server, open browser

3. **Authenticated**
   - Show "Logged in as <display_name>"
   - Fetch user profile from `GET https://api.spotify.com/v1/me`
   - Show basic user info

---

## Dependencies to Add

```toml
[dependencies]
# HTTP client for token exchange
reqwest = { version = "0.12", features = ["json"] }
# Async runtime
tokio = { version = "1", features = ["full"] }
# JSON serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
# Open browser
open = "5"
# Cross-platform config directory
dirs = "6"
# (existing)
base64 = "0.22.1"
color-eyre = "0.6.5"
crossterm = "0.29.0"
rand = "0.10.0"
ratatui = "0.30.0"
sha2 = "0.10.9"
```

---

## File Structure

```
src/
├── main.rs           # Entry point, TUI setup
├── auth/
│   ├── mod.rs        # Auth module exports
│   ├── pkce.rs       # Code verifier/challenge generation
│   ├── server.rs     # Local callback server
│   ├── spotify.rs    # Spotify API client
│   └── token.rs      # Token storage/management
├── tui/
│   ├── mod.rs
│   ├── app.rs        # App state machine
│   └── ui.rs         # Rendering
└── config.rs         # Config paths, constants
```

---

## Flow Diagram

```
┌─────────────────────┐
│   TUI (not logged)  │
│  "Press to login"   │
└──────────┬──────────┘
           │ Enter
           ▼
┌─────────────────────┐
│ Generate PKCE       │
│ verifier + challenge│
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Start local server  │
│ on port 8080        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Open browser with   │
│ Spotify auth URL    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ User clicks "Allow" │
│ in browser          │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Browser redirects   │
│ to localhost:8080   │
│ with ?code=...      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Exchange code for   │
│ access token        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Store tokens to     │
│ ~/.config/tuify/    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ TUI shows:          │
│ "Logged in as X"    │
└─────────────────────┘
```

---

## Next Steps After Authentication

Once logged in, implement:
1. Display current playback state
2. Control playback (play/pause/next/prev)
3. Browse playlists
4. Search for tracks/artists
5. Queue management

---

## Resources

- [Spotify PKCE Flow Docs](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow)
- [Spotify Web API Reference](https://developer.spotify.com/documentation/web-api/reference)
- [PKCE RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636)
