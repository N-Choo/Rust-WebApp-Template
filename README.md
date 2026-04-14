# Rust WebApp Template 🦀

A lightweight, modern web application template built with Rust and **Actix-Web**. It includes a static frontend, a configured Actix server, and custom logging.

## What is Included?

The repository is structured into two main parts: the `client` (Frontend) and the `server` (Backend).

- **`client/`**: Contains the frontend static files.
  - `index.html`: A clean, modern landing page template.
  - `style.css`: A responsive, dark/light mode friendly CSS stylesheet.
- **`server/`**: The Rust Actix-Web backend.
  - **Static File Serving**: Pre-configured to serve the `client/` folder automatically.
  - **Custom Configuration**: Centralized environment variable loading and network setup (`config.rs`).
  - **Custom Logger**: Configured `env_logger` for clean, readable terminal output.

## How to Run

### Prerequisites

Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your machine.

### 1. Start the Server (Development)

To run the server locally, navigate to the `server` directory and run it via Cargo:

```bash
cd server
cargo run
```

By default, the server will start listening on `http://127.0.0.1:3000`.

### 2. Environment Variables

You can customize the server's behavior by passing environment variables:

- `TS_PORT`: The port the server listens on (Default is `3000`).

Example of running with custom variables:

```bash
TS_PORT=8080 cargo run
```

## 🛠 How to Continue (Development)

This template is designed to be a starting point. Here is how to expand it:

1. **Add API Routes:** Open `server/src/routes.rs`. You can uncomment and expand the `api_routes` function to start building your JSON API endpoints under the `/api/` scope.
2. **Add New Handlers:** Create new files in `server/src/handler/` (like `auth.rs` or `users.rs`), add them to `handler/mod.rs`, and bind them to your routes.
3. **Expand the Frontend:**
   Add more HTML, JS, or images to the `client/` folder. The Actix `fs::Files` service will automatically serve anything you place in there.
