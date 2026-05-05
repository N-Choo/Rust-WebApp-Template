# Rust Webapp Template with Entra ID OAuth2

This is a full-stack web application template featuring a high-performance **Rust (Actix-Web)** backend and a modular, lightweight **Vanilla JavaScript** frontend.
It includes a fully functional **OAuth 2.0 Authorization Code Flow** implementation, pre-configured to authenticate users via **Microsoft Entra ID (Azure AD)**. The backend securely handles token exchange and session management, keeping the frontend stateless and fast.

---

## Features

- **Backend:** Rust, Actix-Web, `oauth2` crate for authentication, `moka` for fast in-memory session caching.
- **Frontend:** Modular Vanilla ES6 JavaScript, pure CSS, no heavy frameworks.
- **Auth:** Secure, HTTP-only cookie-based sessions protecting Microsoft Graph API calls.
- **Routing:** Client-side hash routing (`/#/users`, `/#/update`) with an App Shell architecture.

---

## Configuration Setup

Before running the application, you must configure your Microsoft Entra ID credentials. Create an `.env` file at the root of the `server/` directory (or wherever your binary runs) with the following variables:

```env
# Entra ID App Credentials
CLIENT_ID=your-client-id
CLIENT_SECRET=your-client-secret
TENANT_ID=your-tenant-id

# Microsoft OAuth Endpoints
AUTH_URL=[https://login.microsoftonline.com/your-tenant-id/oauth2/v2.0/authorize](https://login.microsoftonline.com/your-tenant-id/oauth2/v2.0/authorize)
TOKEN_URL=[https://login.microsoftonline.com/your-tenant-id/oauth2/v2.0/token](https://login.microsoftonline.com/your-tenant-id/oauth2/v2.0/token)

# Permissions requested during login
OAUTH_SCOPE=User.ReadWrite

# Local Development URL
HOST=http://localhost:3000
```

> **Important:** Ensure your Microsoft Entra ID App Registration has the **Redirect URI** set to `http://localhost:3000/api/auth/callback` for local development.

---

## Project Structure

```text
├── client/                      # Vanilla JS Frontend
│   ├── index.html               # App Shell & Navigation
│   ├── main.js                 # Client-side router & App initialization
│   ├── api.js                 # Centralized API client
│   ├── auth.js                # Authentication helpers
│   ├── users.js               # Users page logic
│   ├── profile.js            # Profile page logic
│   ├── style.css             # Global styles
│   └── pages/                # Page templates
│       ├── welcome.html
│       ├── users.html
│       ├── profile.html
│       └── unauthorized.html
│
├── server/                      # Rust Actix-Web Backend
│   ├── Cargo.toml             # Rust dependencies
│   ├── .env                  # Environment configuration
│   └── src/
│       ├── main.rs           # Server entry point
│       ├── config.rs         # AppConfig, AppState, OAuth & Moka Cache setup
│       ├── routes.rs        # API route definitions
│       ├── handler/         # Request handlers
│       │   ├── auth.rs     # Login, callback, logout handlers
│       │   └── graph.rs    # Graph API proxy handlers
│       ├── services/       # External API clients
│       │   └── graph.rs    # Microsoft Graph API service
│       └── models/         # Data models
│           └── user.rs      # UserProfile, GraphResponse
```

---

## Getting Started

### 1. Prerequisites

- Install [Rust & Cargo](https://rustup.rs/)
- A registered App in Microsoft Entra ID (Azure AD) to get your `CLIENT_ID` and `CLIENT_SECRET`.

### 2. Run the Application

Navigate to the server directory and start the application:

```bash
cd server
cargo run
```

The server will start (defaulting to `http://localhost:3000`). The backend is configured to statically serve the files from the `client/` folder.

### 3. Usage

1. Open your browser and go to `http://localhost:3000`.
2. Click **Login** to be redirected to Microsoft Entra ID.
3. Upon successful login, you will be redirected back to the app with a secure session cookie.
4. You can now view the **Organization Directory** or update your **Profile**, which communicates securely with the Microsoft Graph API.

---

## Architecture

### Backend (Rust + Actix-Web)

The backend follows a layered architecture:

```
┌─────────────────────────────────────────────────┐
│                   routes.rs                     │  Route definitions
├─────────────────────────────────────────────────┤
│           handler/auth.rs  │  handler/graph.rs  │  Request handlers
├─────────────────────────────────────────────────┤
│           services/graph.rs                     │  External API client
├─────────────────────────────────────────────────┤
│              config.rs (AppState)               │  OAuth client + Moka cache
└─────────────────────────────────────────────────┘
```

- **config.rs**: `AppConfig` loads environment, initializes OAuth client, and creates the shared `AppState` with a `moka` cache for session storage.
- **routes.rs**: Defines `/api/auth/*` and `/api/graph/*` routes, plus static file serving.
- **handler/**: Contains request handlers. `auth.rs` handles OAuth flow, `graph.rs` proxies Microsoft Graph API calls.
- **services/graph.rs**: Pure client for Microsoft's Graph API.
- **models/**: Serde-serializable structs (`UserProfile`, `GraphResponse`).

### Frontend (Vanilla JS)

SPA with client-side hash routing (`/#/users`, `/#/profile`):

- **main.js**: App shell, router, and page lazy-loading
- **api.js**: Centralized fetch wrapper with session error handling
- **auth.js**: Auth state management
- **users.js** / **profile.js**: Page-specific logic
- **pages/**: HTML templates

### Authentication Flow

1. **Login**: GET `/api/auth/login` generates a CSRF state, caches it, redirects to Microsoft.
2. **Callback**: Microsoft redirects to `/api/auth/callback` with a code. Server validates CSRF, exchanges code for access token.
3. **Session**: Server creates a UUID `session_id`, maps it to the access token in the Moka cache, sets an `HttpOnly` cookie.
4. **API Requests**: Frontend calls `/api/graph/*`. Backend reads the cookie, retrieves the token from cache, calls Microsoft Graph, returns JSON.

> **Note**: Sessions are stored in-memory via Moka. Restarting the server clears all sessions. For production, replace with Redis.
