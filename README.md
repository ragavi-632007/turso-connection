# Hello World — Rust + Turso DB

No frameworks. No libraries. Pure Rust stdlib + Turso HTTP API.

---

## Folder Structure

```
hello-world/
├── src/
│   ├── main.rs       ← HTTP server (pure stdlib)
│   └── db.rs         ← Turso DB query via curl
├── frontend/
│   └── index.html    ← UI (served by Rust)
├── Cargo.toml
├── .env              ← your Turso credentials
└── .gitignore
```

---

## Step 1 — Get Turso Credentials

1. Go to **https://app.turso.tech**
2. Click your database (`mydb`) → click **"Connect"** button
3. Copy:
   - **URL** → looks like `libsql://mydb-ragavi.turso.io`
   - **Token** → a long JWT string

Paste them into `.env`:
```
TURSO_DATABASE_URL=libsql://mydb-ragavi.turso.io
TURSO_AUTH_TOKEN=paste-your-token-here
```

---

## Step 2 — Create the Table in Turso

1. On app.turso.tech, click **"Edit Data"** next to your `mydb`
2. Run this SQL in the editor:

```sql
CREATE TABLE IF NOT EXISTS hello (message TEXT);
INSERT INTO hello (message) VALUES ('Hello, World! from Turso 🚀');
```

---

## Step 3 — Run the Server

Open terminal in VS Code (`` Ctrl+` ``):

```
cargo run
```

That's it. Open http://localhost:8080 in your browser.

---

## How It Works

- `main.rs` — opens a TCP socket, reads raw HTTP requests, routes them manually
- `db.rs` — calls Turso's HTTPS REST API using `curl` (no TLS library needed)
- `frontend/index.html` — embedded via `include_str!()`, served at `/`
- `/api/hello` — queries Turso, returns `{"message": "..."}` as JSON

---

## Troubleshooting

| Problem | Fix |
|---|---|
| `curl: command not found` | Install curl: `winget install curl` (Windows) |
| `DB not configured` | Check your `.env` file is in the project root |
| `connection error` in browser | Make sure `cargo run` is still running |
| `Hello, World! (from Turso)` but no DB data | Re-run the CREATE TABLE + INSERT SQL in Turso editor |
