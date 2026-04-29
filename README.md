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
├── client/                 # Frontend assets (Static HTML/JS/CSS)
│   ├── index.html          # Main App Shell & Navigation
│   ├── style.css           # Global styles and layout
│   ├── main.js             # Entry point & Client-side router
│   ├── api.js              # Centralized API and Session Error handling
│   └── pages/              # Individual HTML views (welcome, directory, profile)
│
├── server/                 # Rust Actix-Web Backend
│   ├── Cargo.toml          # Rust dependencies
│   ├── src/
│   │   ├── main.rs         # Server initialization
│   │   ├── config.rs       # OAuth client & Moka Cache setup
│   │   ├── routes.rs       # API route definitions
│   │   ├── handler/        # Request handlers (auth.rs, graph.rs)
│   │   ├── services/       # External API logic (Microsoft Graph calls)
│   │   └── models/         # Serde structs for data serialization
│   └── .env                # (Create this file based on the section above)
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

## How the Authentication Works

1. **Login:** Hitting `/api/auth/login` generates a CSRF `state`, caches it, and redirects to Microsoft.
2. **Callback:** Microsoft redirects to `/api/auth/callback` with a `code`. The server validates the CSRF state and exchanges the code for a Microsoft Access Token.
3. **Session:** The server generates a random UUID (`session_id`), maps it to the Access Token in a `moka` memory cache, and sets an `HttpOnly` cookie in the user's browser.
4. **API Calls:** When the frontend requests `/api/graph/users`, the backend reads the cookie, fetches the Entra token from the cache, makes the Graph API request, and returns the JSON to the frontend.

_(Note: Because sessions are currently stored in memory via the `moka` crate, restarting the Rust server will clear all active sessions and log users out. For production, consider swapping the memory cache with a Redis store)._
